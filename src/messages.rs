use protobuf;

use protos::MessageType::*;
use protos::*;

///! In this module we implement the message_type() getter for all protobuf message types.

/// This trait extends the protobuf Message trait to also have a static getter for the message
/// type code.  This getter is implemented in this file for all the messages we use.
pub trait TrezorMessage: protobuf::Message {
	fn message_type() -> MessageType;
}

/// This macro provides the TrezorMessage trait for a protobuf message.
macro_rules! trezor_message_impl {
	($struct:ident, $mtype:expr) => {
		impl TrezorMessage for $struct {
			fn message_type() -> MessageType {
				$mtype
			}
		}
	};
}

trezor_message_impl!(Initialize, MessageType_Initialize);
trezor_message_impl!(Ping, MessageType_Ping);
trezor_message_impl!(Success, MessageType_Success);
trezor_message_impl!(Failure, MessageType_Failure);
trezor_message_impl!(ChangePin, MessageType_ChangePin);
trezor_message_impl!(WipeDevice, MessageType_WipeDevice);
trezor_message_impl!(GetEntropy, MessageType_GetEntropy);
trezor_message_impl!(Entropy, MessageType_Entropy);
trezor_message_impl!(LoadDevice, MessageType_LoadDevice);
trezor_message_impl!(ResetDevice, MessageType_ResetDevice);
trezor_message_impl!(Features, MessageType_Features);
trezor_message_impl!(PinMatrixRequest, MessageType_PinMatrixRequest);
trezor_message_impl!(PinMatrixAck, MessageType_PinMatrixAck);
trezor_message_impl!(Cancel, MessageType_Cancel);
trezor_message_impl!(ApplySettings, MessageType_ApplySettings);
trezor_message_impl!(ButtonRequest, MessageType_ButtonRequest);
trezor_message_impl!(ButtonAck, MessageType_ButtonAck);
trezor_message_impl!(ApplyFlags, MessageType_ApplyFlags);
trezor_message_impl!(BackupDevice, MessageType_BackupDevice);
trezor_message_impl!(EntropyRequest, MessageType_EntropyRequest);
trezor_message_impl!(EntropyAck, MessageType_EntropyAck);
trezor_message_impl!(PassphraseRequest, MessageType_PassphraseRequest);
trezor_message_impl!(PassphraseAck, MessageType_PassphraseAck);
trezor_message_impl!(RecoveryDevice, MessageType_RecoveryDevice);
trezor_message_impl!(WordRequest, MessageType_WordRequest);
trezor_message_impl!(WordAck, MessageType_WordAck);
trezor_message_impl!(GetFeatures, MessageType_GetFeatures);
trezor_message_impl!(SetU2FCounter, MessageType_SetU2FCounter);
trezor_message_impl!(FirmwareErase, MessageType_FirmwareErase);
trezor_message_impl!(FirmwareUpload, MessageType_FirmwareUpload);
trezor_message_impl!(FirmwareRequest, MessageType_FirmwareRequest);
trezor_message_impl!(SelfTest, MessageType_SelfTest);
trezor_message_impl!(GetPublicKey, MessageType_GetPublicKey);
trezor_message_impl!(PublicKey, MessageType_PublicKey);
trezor_message_impl!(SignTx, MessageType_SignTx);
trezor_message_impl!(TxRequest, MessageType_TxRequest);
trezor_message_impl!(TxAck, MessageType_TxAck);
trezor_message_impl!(GetAddress, MessageType_GetAddress);
trezor_message_impl!(Address, MessageType_Address);
trezor_message_impl!(SignMessage, MessageType_SignMessage);
trezor_message_impl!(VerifyMessage, MessageType_VerifyMessage);
trezor_message_impl!(MessageSignature, MessageType_MessageSignature);
trezor_message_impl!(CipherKeyValue, MessageType_CipherKeyValue);
trezor_message_impl!(CipheredKeyValue, MessageType_CipheredKeyValue);
trezor_message_impl!(SignIdentity, MessageType_SignIdentity);
trezor_message_impl!(SignedIdentity, MessageType_SignedIdentity);
trezor_message_impl!(GetECDHSessionKey, MessageType_GetECDHSessionKey);
trezor_message_impl!(ECDHSessionKey, MessageType_ECDHSessionKey);
trezor_message_impl!(CosiCommit, MessageType_CosiCommit);
trezor_message_impl!(CosiCommitment, MessageType_CosiCommitment);
trezor_message_impl!(CosiSign, MessageType_CosiSign);
trezor_message_impl!(CosiSignature, MessageType_CosiSignature);
trezor_message_impl!(DebugLinkDecision, MessageType_DebugLinkDecision);
trezor_message_impl!(DebugLinkGetState, MessageType_DebugLinkGetState);
trezor_message_impl!(DebugLinkState, MessageType_DebugLinkState);
trezor_message_impl!(DebugLinkStop, MessageType_DebugLinkStop);
trezor_message_impl!(DebugLinkLog, MessageType_DebugLinkLog);
trezor_message_impl!(DebugLinkMemoryRead, MessageType_DebugLinkMemoryRead);
trezor_message_impl!(DebugLinkMemory, MessageType_DebugLinkMemory);
trezor_message_impl!(DebugLinkMemoryWrite, MessageType_DebugLinkMemoryWrite);
trezor_message_impl!(DebugLinkFlashErase, MessageType_DebugLinkFlashErase);
trezor_message_impl!(MoneroTransactionInitRequest, MessageType_MoneroTransactionInitRequest);
trezor_message_impl!(MoneroTransactionInitAck, MessageType_MoneroTransactionInitAck);
trezor_message_impl!(
	MoneroTransactionSetInputRequest,
	MessageType_MoneroTransactionSetInputRequest
);
trezor_message_impl!(MoneroTransactionSetInputAck, MessageType_MoneroTransactionSetInputAck);
trezor_message_impl!(
	MoneroTransactionInputsPermutationRequest,
	MessageType_MoneroTransactionInputsPermutationRequest
);
trezor_message_impl!(
	MoneroTransactionInputsPermutationAck,
	MessageType_MoneroTransactionInputsPermutationAck
);
trezor_message_impl!(
	MoneroTransactionInputViniRequest,
	MessageType_MoneroTransactionInputViniRequest
);
trezor_message_impl!(MoneroTransactionInputViniAck, MessageType_MoneroTransactionInputViniAck);
trezor_message_impl!(
	MoneroTransactionAllInputsSetRequest,
	MessageType_MoneroTransactionAllInputsSetRequest
);
trezor_message_impl!(
	MoneroTransactionAllInputsSetAck,
	MessageType_MoneroTransactionAllInputsSetAck
);
trezor_message_impl!(
	MoneroTransactionSetOutputRequest,
	MessageType_MoneroTransactionSetOutputRequest
);
trezor_message_impl!(MoneroTransactionSetOutputAck, MessageType_MoneroTransactionSetOutputAck);
trezor_message_impl!(
	MoneroTransactionAllOutSetRequest,
	MessageType_MoneroTransactionAllOutSetRequest
);
trezor_message_impl!(MoneroTransactionSignInputAck, MessageType_MoneroTransactionSignInputAck);
trezor_message_impl!(MoneroTransactionFinalRequest, MessageType_MoneroTransactionFinalRequest);
trezor_message_impl!(MoneroTransactionFinalAck, MessageType_MoneroTransactionFinalAck);
trezor_message_impl!(MoneroKeyImageExportInitRequest, MessageType_MoneroKeyImageExportInitRequest);
trezor_message_impl!(MoneroKeyImageExportInitAck, MessageType_MoneroKeyImageExportInitAck);
trezor_message_impl!(MoneroKeyImageSyncStepRequest, MessageType_MoneroKeyImageSyncStepRequest);
trezor_message_impl!(MoneroKeyImageSyncStepAck, MessageType_MoneroKeyImageSyncStepAck);
trezor_message_impl!(MoneroKeyImageSyncFinalRequest, MessageType_MoneroKeyImageSyncFinalRequest);
trezor_message_impl!(MoneroKeyImageSyncFinalAck, MessageType_MoneroKeyImageSyncFinalAck);
trezor_message_impl!(MoneroGetAddress, MessageType_MoneroGetAddress);
trezor_message_impl!(MoneroAddress, MessageType_MoneroAddress);
trezor_message_impl!(MoneroGetWatchKey, MessageType_MoneroGetWatchKey);
trezor_message_impl!(MoneroWatchKey, MessageType_MoneroWatchKey);
trezor_message_impl!(DebugMoneroDiagRequest, MessageType_DebugMoneroDiagRequest);
