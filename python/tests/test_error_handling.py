#!/usr/bin/env python3
"""
Error handling tests for sv2-uniffi.

Tests that verify proper error handling and exception raising.
"""

import traceback

def test_error_handling():
    """Test error handling."""
    try:
        from sv2 import (
            Sv2CodecState,
            Sv2CodecError,
            Sv2Encoder,
            Sv2Message,
            OpenExtendedMiningChannel,
        )
        
        # Test invalid key size
        try:
            invalid_key = b"too_short"
            Sv2CodecState.new_initiator(invalid_key)
            print("✗ Error handling test failed - should have thrown exception")
            return False
        except Sv2CodecError:
            # Expected error
            pass
        
        # Note: Random 32-byte keys may or may not fail depending on the cryptographic implementation
        # so we don't test that case to avoid flaky tests

        # Test granular validation signal for invalid fixed-size bytes (U256 must be exactly 32 bytes).
        # In Python this currently surfaces as a conversion-layer error message, not a typed Sv2MessageError.
        try:
            invalid_msg = OpenExtendedMiningChannel(
                request_id=1,
                user_identity="test",
                max_target=b"\xFF" * 31,  # invalid length, should be 32
                nominal_hash_rate=1000.0,
                min_extranonce_size=8,
            )

            # Constructor accepts Python bytes; validation is exercised during FFI conversion.
            wrapped = Sv2Message.OPEN_EXTENDED_MINING_CHANNEL(invalid_msg)
            encoder = Sv2Encoder()
            codec_state = Sv2CodecState.new_initiator(bytes([1]) * 32)
            encoder.encode(wrapped, codec_state)

            print("✗ Error handling test failed - should have thrown conversion error containing ByteArrayLengthMismatch")
            return False
        except Exception as e:
            if "ByteArrayLengthMismatch" not in str(e):
                print(f"✗ Error handling test failed - unexpected error: {type(e).__name__}: {e}")
                return False
        
        print("✓ Error handling test passed")
        return True
    except Exception as e:
        print(f"✗ Error handling test failed: {e}")
        traceback.print_exc()
        return False

if __name__ == "__main__":
    success = test_error_handling()
    exit(0 if success else 1) 