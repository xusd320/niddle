use html5ever::serialize::{self, serialize, SerializeOpts};
use indexmap::IndexMap;
use kuchikiki::ExpandedName;

use crate::serializer::serialize_text_only;

use super::NodeRepr;

#[napi]
impl NodeRepr {
  /// Select the the fist node that match the given css selector, like document.querySelector.
  #[napi]
  pub fn select(&self, selectors: String) -> Option<NodeRepr> {
    self.0.select_first(&selectors).ok().map(Into::into)
  }

  /// Select all nodes that match the given css selector, like document.querySelectorAll.
  #[napi]
  pub fn select_all(&self, selectors: String) -> Vec<NodeRepr> {
    self
      .0
      .select(&selectors)
      .map_or(vec![], |els| els.map(Into::into).collect())
  }

  /// Get all children nodes of this node.
  #[napi]
  pub fn get_children(&self) -> Vec<NodeRepr> {
    self.0.children().map(Into::into).collect()
  }

  /// Get attribute value of this node by given name.
  #[napi]
  pub fn get_attribute(&self, name: String) -> Option<String> {
    self
      .0
      .as_element()
      .and_then(|e| e.attributes.borrow().get(name).map(|v| v.to_string()))
  }

  /// Get attributes K-V object of this node.
  #[napi]
  pub fn get_attributes(&self) -> IndexMap<String, String> {
    self.0.as_element().map_or_else(IndexMap::new, |e| {
      e.attributes
        .borrow()
        .map
        .iter()
        .map(|(expanded_name, attr)| {
          let ExpandedName { local, ns: _ } = expanded_name;
          (local.to_string(), attr.value.to_string())
        })
        .collect::<IndexMap<String, String>>()
    })
  }

  /// Get the serialized html of this node, including its all descendants and itelf;.
  #[napi]
  pub fn outer_html(&self) -> String {
    let mut u8_vec = Vec::new();
    serialize(
      &mut u8_vec,
      self,
      SerializeOpts {
        traversal_scope: serialize::TraversalScope::IncludeNode,
        create_missing_parent: false,
        scripting_enabled: true,
      },
    )
    .unwrap();
    unsafe { String::from_utf8_unchecked(u8_vec) }
  }

  /// Get the serialized html of this node, only including its all descendants;.
  #[napi]
  pub fn inner_html(&self) -> String {
    let mut buf = Vec::<u8>::new();
    serialize(
      &mut buf,
      self,
      SerializeOpts {
        traversal_scope: serialize::TraversalScope::ChildrenOnly(None),
        create_missing_parent: false,
        scripting_enabled: true,
      },
    )
    .unwrap();
    unsafe { String::from_utf8_unchecked(buf) }
  }

  /// Get all text nodes content of this node, including its all descendants and itelf;.
  #[napi]
  pub fn text(&self) -> String {
    let mut buf = Vec::<u8>::new();
    serialize_text_only(&self.0, &mut buf).unwrap();
    unsafe { String::from_utf8_unchecked(buf) }
  }
}
