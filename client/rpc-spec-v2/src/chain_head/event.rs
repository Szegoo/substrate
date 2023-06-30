// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! The chain head's event returned as json compatible object.

use serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer};
use sp_api::ApiError;
use sp_version::RuntimeVersion;
use std::num::NonZeroUsize;

/// The network config parameter is used when a function
/// needs to request the information from its peers.
///
/// These values can be tweaked depending on the urgency of the JSON-RPC function call.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkConfig {
	/// The total number of peers from which the information is requested.
	total_attempts: u64,
	/// The maximum number of requests to perform in parallel.
	///
	/// # Note
	///
	/// A zero value is illegal.
	max_parallel: NonZeroUsize,
	/// The time, in milliseconds, after which a single requests towards one peer
	/// is considered unsuccessful.
	timeout_ms: u64,
}

/// The operation could not be processed due to an error.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorEvent {
	/// Reason of the error.
	pub error: String,
}

/// The runtime specification of the current block.
///
/// This event is generated for:
///   - the first announced block by the follow subscription
///   - blocks that suffered a change in runtime compared with their parents
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeVersionEvent {
	/// The runtime version.
	pub spec: RuntimeVersion,
}

/// The runtime event generated if the `follow` subscription
/// has set the `with_runtime` flag.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum RuntimeEvent {
	/// The runtime version of this block.
	Valid(RuntimeVersionEvent),
	/// The runtime could not be obtained due to an error.
	Invalid(ErrorEvent),
}

impl From<ApiError> for RuntimeEvent {
	fn from(err: ApiError) -> Self {
		RuntimeEvent::Invalid(ErrorEvent { error: format!("Api error: {}", err) })
	}
}

/// Contain information about the latest finalized block.
///
/// # Note
///
/// This is the first event generated by the `follow` subscription
/// and is submitted only once.
///
/// If the `with_runtime` flag is set, then this event contains
/// the `RuntimeEvent`, otherwise the `RuntimeEvent` is not present.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Initialized<Hash> {
	/// The hash of the latest finalized block.
	pub finalized_block_hash: Hash,
	/// The runtime version of the finalized block.
	///
	/// # Note
	///
	/// This is present only if the `with_runtime` flag is set for
	/// the `follow` subscription.
	pub finalized_block_runtime: Option<RuntimeEvent>,
	/// Privately keep track if the `finalized_block_runtime` should be
	/// serialized.
	#[serde(default)]
	pub(crate) with_runtime: bool,
}

impl<Hash: Serialize> Serialize for Initialized<Hash> {
	/// Custom serialize implementation to include the `RuntimeEvent` depending
	/// on the internal `with_runtime` flag.
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		if self.with_runtime {
			let mut state = serializer.serialize_struct("Initialized", 2)?;
			state.serialize_field("finalizedBlockHash", &self.finalized_block_hash)?;
			state.serialize_field("finalizedBlockRuntime", &self.finalized_block_runtime)?;
			state.end()
		} else {
			let mut state = serializer.serialize_struct("Initialized", 1)?;
			state.serialize_field("finalizedBlockHash", &self.finalized_block_hash)?;
			state.end()
		}
	}
}

/// Indicate a new non-finalized block.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewBlock<Hash> {
	/// The hash of the new block.
	pub block_hash: Hash,
	/// The parent hash of the new block.
	pub parent_block_hash: Hash,
	/// The runtime version of the new block.
	///
	/// # Note
	///
	/// This is present only if the `with_runtime` flag is set for
	/// the `follow` subscription.
	pub new_runtime: Option<RuntimeEvent>,
	/// Privately keep track if the `finalized_block_runtime` should be
	/// serialized.
	#[serde(default)]
	pub(crate) with_runtime: bool,
}

impl<Hash: Serialize> Serialize for NewBlock<Hash> {
	/// Custom serialize implementation to include the `RuntimeEvent` depending
	/// on the internal `with_runtime` flag.
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		if self.with_runtime {
			let mut state = serializer.serialize_struct("NewBlock", 3)?;
			state.serialize_field("blockHash", &self.block_hash)?;
			state.serialize_field("parentBlockHash", &self.parent_block_hash)?;
			state.serialize_field("newRuntime", &self.new_runtime)?;
			state.end()
		} else {
			let mut state = serializer.serialize_struct("NewBlock", 2)?;
			state.serialize_field("blockHash", &self.block_hash)?;
			state.serialize_field("parentBlockHash", &self.parent_block_hash)?;
			state.end()
		}
	}
}

/// Indicate the block hash of the new best block.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BestBlockChanged<Hash> {
	/// The block hash of the new best block.
	pub best_block_hash: Hash,
}

/// Indicate the finalized and pruned block hashes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Finalized<Hash> {
	/// Block hashes that are finalized.
	pub finalized_block_hashes: Vec<Hash>,
	/// Block hashes that are pruned (removed).
	pub pruned_block_hashes: Vec<Hash>,
}

/// The event generated by the `follow` method.
///
/// The events are generated in the following order:
/// 1. Initialized - generated only once to signal the latest finalized block
/// 2. NewBlock - a new block was added.
/// 3. BestBlockChanged - indicate that the best block is now the one from this event. The block was
///    announced priorly with the `NewBlock` event.
/// 4. Finalized - State the finalized and pruned blocks.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "event")]
pub enum FollowEvent<Hash> {
	/// The latest finalized block.
	///
	/// This event is generated only once.
	Initialized(Initialized<Hash>),
	/// A new non-finalized block was added.
	NewBlock(NewBlock<Hash>),
	/// The best block of the chain.
	BestBlockChanged(BestBlockChanged<Hash>),
	/// A list of finalized and pruned blocks.
	Finalized(Finalized<Hash>),
	/// The subscription is dropped and no further events
	/// will be generated.
	Stop,
}

/// The result of a chain head method.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChainHeadResult<T> {
	/// Result of the method.
	pub result: T,
}

/// The event generated by the body / call / storage methods.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "event")]
pub enum ChainHeadEvent<T> {
	/// The request completed successfully.
	Done(ChainHeadResult<T>),
	/// The resources requested are inaccessible.
	///
	/// Resubmitting the request later might succeed.
	Inaccessible(ErrorEvent),
	/// An error occurred. This is definitive.
	Error(ErrorEvent),
	/// The provided subscription ID is stale or invalid.
	Disjoint,
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn follow_initialized_event_no_updates() {
		// Runtime flag is false.
		let event: FollowEvent<String> = FollowEvent::Initialized(Initialized {
			finalized_block_hash: "0x1".into(),
			finalized_block_runtime: None,
			with_runtime: false,
		});

		let ser = serde_json::to_string(&event).unwrap();
		let exp = r#"{"event":"initialized","finalizedBlockHash":"0x1"}"#;
		assert_eq!(ser, exp);

		let event_dec: FollowEvent<String> = serde_json::from_str(exp).unwrap();
		assert_eq!(event_dec, event);
	}

	#[test]
	fn follow_initialized_event_with_updates() {
		// Runtime flag is true, block runtime must always be reported for this event.
		let runtime = RuntimeVersion {
			spec_name: "ABC".into(),
			impl_name: "Impl".into(),
			spec_version: 1,
			..Default::default()
		};

		let runtime_event = RuntimeEvent::Valid(RuntimeVersionEvent { spec: runtime });
		let mut initialized = Initialized {
			finalized_block_hash: "0x1".into(),
			finalized_block_runtime: Some(runtime_event),
			with_runtime: true,
		};
		let event: FollowEvent<String> = FollowEvent::Initialized(initialized.clone());

		let ser = serde_json::to_string(&event).unwrap();
		let exp = concat!(
			r#"{"event":"initialized","finalizedBlockHash":"0x1","#,
			r#""finalizedBlockRuntime":{"type":"valid","spec":{"specName":"ABC","implName":"Impl","authoringVersion":0,"#,
			r#""specVersion":1,"implVersion":0,"apis":[],"transactionVersion":0,"stateVersion":0}}}"#,
		);
		assert_eq!(ser, exp);

		let event_dec: FollowEvent<String> = serde_json::from_str(exp).unwrap();
		// The `with_runtime` field is used for serialization purposes.
		initialized.with_runtime = false;
		assert!(matches!(
			event_dec, FollowEvent::Initialized(ref dec) if dec == &initialized
		));
	}

	#[test]
	fn follow_new_block_event_no_updates() {
		// Runtime flag is false.
		let event: FollowEvent<String> = FollowEvent::NewBlock(NewBlock {
			block_hash: "0x1".into(),
			parent_block_hash: "0x2".into(),
			new_runtime: None,
			with_runtime: false,
		});

		let ser = serde_json::to_string(&event).unwrap();
		let exp = r#"{"event":"newBlock","blockHash":"0x1","parentBlockHash":"0x2"}"#;
		assert_eq!(ser, exp);

		let event_dec: FollowEvent<String> = serde_json::from_str(exp).unwrap();
		assert_eq!(event_dec, event);
	}

	#[test]
	fn follow_new_block_event_with_updates() {
		// Runtime flag is true, block runtime must always be reported for this event.
		let runtime = RuntimeVersion {
			spec_name: "ABC".into(),
			impl_name: "Impl".into(),
			spec_version: 1,
			..Default::default()
		};

		let runtime_event = RuntimeEvent::Valid(RuntimeVersionEvent { spec: runtime });
		let mut new_block = NewBlock {
			block_hash: "0x1".into(),
			parent_block_hash: "0x2".into(),
			new_runtime: Some(runtime_event),
			with_runtime: true,
		};

		let event: FollowEvent<String> = FollowEvent::NewBlock(new_block.clone());

		let ser = serde_json::to_string(&event).unwrap();
		let exp = concat!(
			r#"{"event":"newBlock","blockHash":"0x1","parentBlockHash":"0x2","#,
			r#""newRuntime":{"type":"valid","spec":{"specName":"ABC","implName":"Impl","authoringVersion":0,"#,
			r#""specVersion":1,"implVersion":0,"apis":[],"transactionVersion":0,"stateVersion":0}}}"#,
		);
		assert_eq!(ser, exp);

		let event_dec: FollowEvent<String> = serde_json::from_str(exp).unwrap();
		// The `with_runtime` field is used for serialization purposes.
		new_block.with_runtime = false;
		assert!(matches!(
			event_dec, FollowEvent::NewBlock(ref dec) if dec == &new_block
		));

		// Runtime flag is true, runtime didn't change compared to parent.
		let mut new_block = NewBlock {
			block_hash: "0x1".into(),
			parent_block_hash: "0x2".into(),
			new_runtime: None,
			with_runtime: true,
		};
		let event: FollowEvent<String> = FollowEvent::NewBlock(new_block.clone());

		let ser = serde_json::to_string(&event).unwrap();
		let exp =
			r#"{"event":"newBlock","blockHash":"0x1","parentBlockHash":"0x2","newRuntime":null}"#;
		assert_eq!(ser, exp);
		new_block.with_runtime = false;
		let event_dec: FollowEvent<String> = serde_json::from_str(exp).unwrap();
		assert!(matches!(
			event_dec, FollowEvent::NewBlock(ref dec) if dec == &new_block
		));
	}

	#[test]
	fn follow_best_block_changed_event() {
		let event: FollowEvent<String> =
			FollowEvent::BestBlockChanged(BestBlockChanged { best_block_hash: "0x1".into() });

		let ser = serde_json::to_string(&event).unwrap();
		let exp = r#"{"event":"bestBlockChanged","bestBlockHash":"0x1"}"#;
		assert_eq!(ser, exp);

		let event_dec: FollowEvent<String> = serde_json::from_str(exp).unwrap();
		assert_eq!(event_dec, event);
	}

	#[test]
	fn follow_finalized_event() {
		let event: FollowEvent<String> = FollowEvent::Finalized(Finalized {
			finalized_block_hashes: vec!["0x1".into()],
			pruned_block_hashes: vec!["0x2".into()],
		});

		let ser = serde_json::to_string(&event).unwrap();
		let exp =
			r#"{"event":"finalized","finalizedBlockHashes":["0x1"],"prunedBlockHashes":["0x2"]}"#;
		assert_eq!(ser, exp);

		let event_dec: FollowEvent<String> = serde_json::from_str(exp).unwrap();
		assert_eq!(event_dec, event);
	}

	#[test]
	fn follow_stop_event() {
		let event: FollowEvent<String> = FollowEvent::Stop;

		let ser = serde_json::to_string(&event).unwrap();
		let exp = r#"{"event":"stop"}"#;
		assert_eq!(ser, exp);

		let event_dec: FollowEvent<String> = serde_json::from_str(exp).unwrap();
		assert_eq!(event_dec, event);
	}

	#[test]
	fn chain_head_done_event() {
		let event: ChainHeadEvent<String> =
			ChainHeadEvent::Done(ChainHeadResult { result: "A".into() });

		let ser = serde_json::to_string(&event).unwrap();
		let exp = r#"{"event":"done","result":"A"}"#;
		assert_eq!(ser, exp);

		let event_dec: ChainHeadEvent<String> = serde_json::from_str(exp).unwrap();
		assert_eq!(event_dec, event);
	}

	#[test]
	fn chain_head_inaccessible_event() {
		let event: ChainHeadEvent<String> =
			ChainHeadEvent::Inaccessible(ErrorEvent { error: "A".into() });

		let ser = serde_json::to_string(&event).unwrap();
		let exp = r#"{"event":"inaccessible","error":"A"}"#;
		assert_eq!(ser, exp);

		let event_dec: ChainHeadEvent<String> = serde_json::from_str(exp).unwrap();
		assert_eq!(event_dec, event);
	}

	#[test]
	fn chain_head_error_event() {
		let event: ChainHeadEvent<String> = ChainHeadEvent::Error(ErrorEvent { error: "A".into() });

		let ser = serde_json::to_string(&event).unwrap();
		let exp = r#"{"event":"error","error":"A"}"#;
		assert_eq!(ser, exp);

		let event_dec: ChainHeadEvent<String> = serde_json::from_str(exp).unwrap();
		assert_eq!(event_dec, event);
	}

	#[test]
	fn chain_head_disjoint_event() {
		let event: ChainHeadEvent<String> = ChainHeadEvent::Disjoint;

		let ser = serde_json::to_string(&event).unwrap();
		let exp = r#"{"event":"disjoint"}"#;
		assert_eq!(ser, exp);

		let event_dec: ChainHeadEvent<String> = serde_json::from_str(exp).unwrap();
		assert_eq!(event_dec, event);
	}

	#[test]
	fn chain_head_network_config() {
		let conf = NetworkConfig {
			total_attempts: 1,
			max_parallel: NonZeroUsize::new(2).expect("Non zero number; qed"),
			timeout_ms: 3,
		};

		let ser = serde_json::to_string(&conf).unwrap();
		let exp = r#"{"totalAttempts":1,"maxParallel":2,"timeoutMs":3}"#;
		assert_eq!(ser, exp);

		let conf_dec: NetworkConfig = serde_json::from_str(exp).unwrap();
		assert_eq!(conf_dec, conf);
	}
}
