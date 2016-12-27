initSidebarItems({"macro":[["forward_to_deserialize","Helper to forward `Deserializer` methods to `Deserializer::deserialize`. Every given method ignores all arguments and forwards to `deserialize`. Note that `deserialize_enum` simply returns an `Error::invalid_type`; a better approach is tracked in [serde-rs/serde#521][1]."]],"mod":[["bytes","Helper module to enable serializing bytes more efficiently"],["de","Generic deserialization framework."],["iter","Module that contains helper iterators."],["ser","Generic serialization framework. # For Developers who want to serialize objects Implement the `Serialize` trait for the type of objects you want to serialize. Call methods of the `serializer` object. For which methods to call and how to do so, look at the documentation of the `Serializer` trait."]]});