//! Integration tests for the "Ownership-is-Capability" model.
//!
//! These tests verify the invariants documented in the architecture design:
//! * Capability minting and rights checking.
//! * Rights-subset enforcement during derivation.
//! * CNode slot management.
//! * Badge allocator uniqueness.
//! * Deterministic revocation via `Drop`.

use capability::{Badge, BadgeAllocator, Cap, CapObject, CapError, CNode, Rights};

// ── Test fixture ─────────────────────────────────────────────────────────────

/// A trivial kernel object used solely for testing.
#[derive(Debug)]
struct DummyObj {
    revoked: bool,
}

impl DummyObj {
    fn new() -> Self {
        Self { revoked: false }
    }
}

impl CapObject for DummyObj {
    fn on_revoke(&mut self) {
        self.revoked = true;
    }
}

fn dummy_cap(rights: Rights) -> Cap<DummyObj> {
    Cap::mint(DummyObj::new(), rights, Badge::NONE)
}

// ── Rights tests ──────────────────────────────────────────────────────────────

#[test]
fn rights_subset_reflexive() {
    let r = Rights::READ | Rights::WRITE;
    assert!(r.is_subset_of(r), "rights should be a subset of themselves");
}

#[test]
fn rights_subset_strict() {
    let read = Rights::READ;
    let all  = Rights::ALL;
    assert!(read.is_subset_of(all));
    assert!(!all.is_subset_of(read));
}

#[test]
fn rights_empty_is_subset_of_everything() {
    assert!(Rights::empty().is_subset_of(Rights::ALL));
    assert!(Rights::empty().is_subset_of(Rights::empty()));
}

// ── Cap::mint / has_rights ────────────────────────────────────────────────────

#[test]
fn cap_mint_stores_rights_and_badge() {
    let badge = Badge::new(42);
    let cap   = Cap::mint(DummyObj::new(), Rights::READ, badge);
    assert_eq!(cap.rights(), Rights::READ);
    assert_eq!(cap.badge(),  badge);
}

#[test]
fn cap_has_rights_true_when_present() {
    let cap = dummy_cap(Rights::READ | Rights::WRITE);
    assert!(cap.has_rights(Rights::READ));
    assert!(cap.has_rights(Rights::WRITE));
    assert!(!cap.has_rights(Rights::GRANT));
}

// ── Cap::derive ───────────────────────────────────────────────────────────────

#[test]
fn derive_with_subset_rights_succeeds() {
    let parent = dummy_cap(Rights::ALL);
    let child  = parent.derive(Rights::READ, Badge::new(1)).unwrap();
    assert_eq!(child.rights(), Rights::READ);
}

#[test]
fn derive_rights_escalation_is_rejected() {
    // Parent has only READ; child attempts READ|WRITE — must fail.
    let parent = dummy_cap(Rights::READ);
    let err    = parent.derive(Rights::READ | Rights::WRITE, Badge::NONE).unwrap_err();
    assert_eq!(err, CapError::RightsEscalation);
}

#[test]
fn derive_without_grant_right_is_rejected() {
    // Parent has READ|WRITE but not GRANT.
    let parent = dummy_cap(Rights::READ | Rights::WRITE);
    let err    = parent.derive(Rights::READ, Badge::NONE).unwrap_err();
    assert_eq!(err, CapError::InsufficientRights);
}

#[test]
fn derive_same_rights_succeeds_with_grant() {
    let parent = dummy_cap(Rights::READ | Rights::GRANT);
    let child  = parent.derive(Rights::READ, Badge::NONE).unwrap();
    assert_eq!(child.rights(), Rights::READ);
}

// ── Cap::borrow_object ────────────────────────────────────────────────────────

#[test]
fn borrow_object_succeeds_with_sufficient_rights() {
    let cap = dummy_cap(Rights::READ | Rights::WRITE);
    assert!(cap.borrow_object(Rights::READ).is_ok());
}

#[test]
fn borrow_object_fails_with_insufficient_rights() {
    let cap = dummy_cap(Rights::READ); // no WRITE
    assert_eq!(
        cap.borrow_object(Rights::WRITE).unwrap_err(),
        CapError::InsufficientRights
    );
}

#[test]
fn borrow_object_mut_succeeds_with_write_rights() {
    let mut cap = dummy_cap(Rights::WRITE);
    assert!(cap.borrow_object_mut(Rights::WRITE).is_ok());
}

// ── Cap::into_object ─────────────────────────────────────────────────────────

#[test]
fn into_object_extracts_underlying_object() {
    let cap = Cap::mint(DummyObj::new(), Rights::ALL, Badge::NONE);
    let obj = cap.into_object();
    // `revoked` must be false — on_revoke was NOT called because we bypassed drop.
    assert!(!obj.revoked);
}

// ── Drop / revocation ─────────────────────────────────────────────────────────

#[test]
fn drop_calls_on_revoke() {
    use core::cell::Cell;

    // We need a flag outside the object to observe revocation after drop.
    // Use a static Cell trick: wrap in a newtype that writes to an external flag.
    struct WatchedObj<'a>(&'a Cell<bool>);
    impl<'a> CapObject for WatchedObj<'a> {
        fn on_revoke(&mut self) { self.0.set(true); }
    }

    let revoked = Cell::new(false);
    let cap = Cap::mint(WatchedObj(&revoked), Rights::ALL, Badge::NONE);
    assert!(!revoked.get());
    drop(cap);
    assert!(revoked.get(), "on_revoke must be called on drop");
}

// ── CNode ─────────────────────────────────────────────────────────────────────

#[test]
fn cnode_insert_and_remove() {
    let mut cnode: CNode<DummyObj, 4> = CNode::new();
    assert_eq!(cnode.occupied_count(), 0);

    let cap = dummy_cap(Rights::ALL);
    cnode.insert(0, cap).unwrap();
    assert_eq!(cnode.occupied_count(), 1);

    let removed = cnode.remove(0).unwrap();
    assert_eq!(removed.rights(), Rights::ALL);
    assert_eq!(cnode.occupied_count(), 0);
}

#[test]
fn cnode_insert_occupied_slot_fails() {
    let mut cnode: CNode<DummyObj, 4> = CNode::new();
    cnode.insert(0, dummy_cap(Rights::ALL)).unwrap();
    let err = cnode.insert(0, dummy_cap(Rights::READ)).unwrap_err();
    assert_eq!(err, CapError::SlotOccupied);
}

#[test]
fn cnode_remove_empty_slot_fails() {
    let mut cnode: CNode<DummyObj, 4> = CNode::new();
    let err = cnode.remove(0).unwrap_err();
    assert_eq!(err, CapError::SlotEmpty);
}

#[test]
fn cnode_out_of_range_fails() {
    let mut cnode: CNode<DummyObj, 4> = CNode::new();
    assert_eq!(
        cnode.insert(99, dummy_cap(Rights::ALL)).unwrap_err(),
        CapError::SlotOutOfRange
    );
    assert_eq!(cnode.remove(99).unwrap_err(), CapError::SlotOutOfRange);
    assert_eq!(cnode.borrow(99).unwrap_err(),  CapError::SlotOutOfRange);
}

#[test]
fn cnode_borrow_reflects_rights() {
    let mut cnode: CNode<DummyObj, 4> = CNode::new();
    cnode.insert(2, dummy_cap(Rights::READ)).unwrap();
    let cap_ref = cnode.borrow(2).unwrap();
    assert_eq!(cap_ref.rights(), Rights::READ);
}

// ── BadgeAllocator ────────────────────────────────────────────────────────────

#[test]
fn badge_allocator_produces_unique_values() {
    let mut alloc = BadgeAllocator::new();
    let badges: Vec<_> = (0..10).map(|_| alloc.alloc().raw()).collect();
    let unique: std::collections::HashSet<_> = badges.iter().copied().collect();
    assert_eq!(badges.len(), unique.len(), "all allocated badges must be unique");
}

#[test]
fn badge_allocator_starts_at_one() {
    let mut alloc = BadgeAllocator::new();
    assert_eq!(alloc.alloc().raw(), 1);
    assert_eq!(alloc.alloc().raw(), 2);
}

#[test]
fn badge_allocator_derive_uses_fresh_badge() {
    let mut alloc = BadgeAllocator::new();
    let parent = dummy_cap(Rights::ALL);
    let child  = alloc.derive_with_fresh_badge(parent, Rights::READ).unwrap();
    assert!(!child.badge().is_none()); // badge must not be NONE
    assert_eq!(child.rights(), Rights::READ);
}

// ── Badge ─────────────────────────────────────────────────────────────────────

#[test]
fn badge_none_is_zero() {
    assert_eq!(Badge::NONE.raw(), 0);
    assert!(Badge::NONE.is_none());
}

#[test]
fn badge_nonzero_is_not_none() {
    let b = Badge::new(7);
    assert!(!b.is_none());
    assert_eq!(b.raw(), 7);
}
