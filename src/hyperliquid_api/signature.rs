// use ethers::{
//     contract::{Eip712, EthAbiType},
//     types::{transaction::eip712::Eip712, H256},
// };

// pub(crate) mod l1 {
//     use super::*;
//     #[derive(Debug, Eip712, Clone, EthAbiType)]
//     #[eip712(
//         name = "Exchange",
//         version = "1",
//         chain_id = 1337,
//         verifying_contract = "0x0000000000000000000000000000000000000000"
//     )]
//     pub(crate) struct Agent {
//         pub(crate) source: String,
//         pub(crate) connection_id: H256,
//     }
// }

// use ethers::{
//     core::k256::{
//         ecdsa::{recoverable, signature::DigestSigner},
//         elliptic_curve::FieldBytes,
//         Secp256k1,
//     },
//     signers::LocalWallet,
//     types::{Signature, U256},
// };

// // For synchronous signing.
// // Needed to duplicate our own copy because it wasn't possible to import from ethers-signers.
// use ethers::core::k256::ecdsa::signature::digest::{
//     FixedOutput, FixedOutputReset, HashMarker, Reset, Update,
// };
// use ethers::prelude::k256::{
//     elliptic_curve::generic_array::GenericArray,
//     sha2::{
//         self,
//         digest::{Output, OutputSizeUser},
//         Digest,
//     },
// };

// pub(crate) type Sha256Proxy = ProxyDigest<sha2::Sha256>;

// #[derive(Clone)]
// pub(crate) enum ProxyDigest<D: Digest> {
//     Proxy(Output<D>),
//     Digest(D),
// }

// impl<D: Digest + Clone> From<H256> for ProxyDigest<D>
// where
//     GenericArray<u8, <D as OutputSizeUser>::OutputSize>: Copy,
// {
//     fn from(src: H256) -> Self {
//         ProxyDigest::Proxy(*GenericArray::from_slice(src.as_bytes()))
//     }
// }

// impl<D: Digest> Default for ProxyDigest<D> {
//     fn default() -> Self {
//         ProxyDigest::Digest(D::new())
//     }
// }

// impl<D: Digest> Update for ProxyDigest<D> {
//     // we update only if we are digest
//     fn update(&mut self, data: &[u8]) {
//         match self {
//             ProxyDigest::Digest(ref mut d) => {
//                 d.update(data);
//             }
//             ProxyDigest::Proxy(..) => {
//                 unreachable!("can not update if we are proxy");
//             }
//         }
//     }
// }

// impl<D: Digest> HashMarker for ProxyDigest<D> {}

// impl<D: Digest> Reset for ProxyDigest<D> {
//     // make new one
//     fn reset(&mut self) {
//         *self = Self::default();
//     }
// }

// impl<D: Digest> OutputSizeUser for ProxyDigest<D> {
//     // we default to the output of the original digest
//     type OutputSize = <D as OutputSizeUser>::OutputSize;
// }

// impl<D: Digest> FixedOutput for ProxyDigest<D> {
//     fn finalize_into(self, out: &mut GenericArray<u8, Self::OutputSize>) {
//         match self {
//             ProxyDigest::Digest(d) => {
//                 *out = d.finalize();
//             }
//             ProxyDigest::Proxy(p) => {
//                 *out = p;
//             }
//         }
//     }
// }

// impl<D: Digest> FixedOutputReset for ProxyDigest<D> {
//     fn finalize_into_reset(&mut self, out: &mut Output<Self>) {
//         let s = std::mem::take(self);
//         Digest::finalize_into(s, out)
//     }
// }
// pub(crate) fn sign_l1_action(
//     wallet: &LocalWallet,
//     connection_id: H256,
//     is_mainnet: bool,
// ) -> anyhow::Result<Signature> {
//     let source = if is_mainnet { "a" } else { "b" }.to_string();
//     sign_typed_data(
//         &l1::Agent {
//             source,
//             connection_id,
//         },
//         wallet,
//     )
// }

// pub(crate) fn sign_typed_data<T: Eip712>(
//     payload: &T,
//     wallet: &LocalWallet,
// ) -> anyhow::Result<Signature> {
//     let encoded = payload.clone().encode_eip712()?;

//     Ok(sign_hash(H256::from(encoded), wallet))
// }

// fn sign_hash(hash: H256, wallet: &LocalWallet) -> Signature {
//     let recoverable_sig: recoverable::Signature =
//         wallet.signer().sign_digest(Sha256Proxy::from(hash));

//     let v = u8::from(recoverable_sig.recovery_id()) as u64 + 27;

//     let r_bytes: FieldBytes<Secp256k1> = recoverable_sig.r().into();
//     let s_bytes: FieldBytes<Secp256k1> = recoverable_sig.s().into();
//     let r = U256::from_big_endian(r_bytes.as_slice());
//     let s = U256::from_big_endian(s_bytes.as_slice());
//     Signature::

//     Signature { r, s, v }
// }
