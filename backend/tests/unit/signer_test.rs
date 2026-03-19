use bunker::signer::Signer;
use nostr::prelude::*;

#[tokio::test]
async fn test_sign_event_valid() {
    let keys = Keys::generate();
    let signer = Signer::new(keys.secret_key().clone());

    let content = "Hello, Nostr!";
    let unsigned_event = EventBuilder::text_note(content).build(keys.public_key());

    let signed_event = signer.sign_event(unsigned_event).await.expect("Should sign event");

    // Verify event ID, pubkey, and signature
    assert_eq!(signed_event.pubkey, keys.public_key());
    assert_eq!(signed_event.content, content);
    assert!(signed_event.verify().is_ok());
}

#[tokio::test]
async fn test_sign_event_with_known_keys() {
    // Generated fresh for testing
    let secret_key = SecretKey::parse("1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef").unwrap();
    let keys = Keys::new(secret_key);
    let signer = Signer::new(keys.secret_key().clone());

    let content = "Signing test";
    let unsigned_event = EventBuilder::text_note(content).build(keys.public_key());

    let signed_event = signer.sign_event(unsigned_event).await.expect("Should sign event");

    // Verify the signature cryptographically
    assert!(signed_event.verify().is_ok());
    assert_eq!(signed_event.pubkey, keys.public_key());
}
