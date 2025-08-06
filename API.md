**niddle** • **Docs**

***

# niddle

## Classes

### NodeRepr

#### Constructors

##### new NodeRepr()

> **new NodeRepr**(): [`NodeRepr`](README.md#noderepr)

The node object, cann't be instantiated in javascript. So call the constructor will throw an error.

###### Returns

[`NodeRepr`](README.md#noderepr)

###### Defined in

[index.d.ts:137](https://github.com/umijs/niddle/blob/main/index.d.ts#L137)

#### Methods

##### append()

> **append**(`newChild`): `void`

Append a child node to this node, after existing children.

The child node will be remove from its previous position.

###### Parameters

| Parameter | Type |
| ------ | ------ |
| `newChild` | [`NodeRepr`](README.md#noderepr) |

###### Returns

`void`

###### Defined in

[index.d.ts:18](https://github.com/umijs/niddle/blob/main/index.d.ts#L18)

##### appendSequence()

> **appendSequence**(`newChildren`): `void`

Append some children nodes to this node by order, after existing children.

These children nodes will be remove from their previous position.

###### Parameters

| Parameter | Type |
| ------ | ------ |
| `newChildren` | [`NodeRepr`](README.md#noderepr)[] |

###### Returns

`void`

###### Defined in

[index.d.ts:25](https://github.com/umijs/niddle/blob/main/index.d.ts#L25)

##### clone()

> **clone**(): [`NodeRepr`](README.md#noderepr)

Clone this node to a new instance, not clone its descendants.

###### Returns

[`NodeRepr`](README.md#noderepr)

###### Defined in

[index.d.ts:142](https://github.com/umijs/niddle/blob/main/index.d.ts#L142)

##### cloneRecursive()

> **cloneRecursive**(): [`NodeRepr`](README.md#noderepr)

Clone this node to a new instance, including its all descendants.

###### Returns

[`NodeRepr`](README.md#noderepr)

###### Defined in

[index.d.ts:147](https://github.com/umijs/niddle/blob/main/index.d.ts#L147)

##### getAttribute()

> **getAttribute**(`name`): `string`

Get attribute value of this node by given name.

###### Parameters

| Parameter | Type |
| ------ | ------ |
| `name` | `string` |

###### Returns

`string`

###### Defined in

[index.d.ts:112](https://github.com/umijs/niddle/blob/main/index.d.ts#L112)

##### getAttributes()

> **getAttributes**(): `Record`\<`string`, `string`\>

Get attributes K-V object of this node.

###### Returns

`Record`\<`string`, `string`\>

###### Defined in

[index.d.ts:117](https://github.com/umijs/niddle/blob/main/index.d.ts#L117)

##### getChildren()

> **getChildren**(): [`NodeRepr`](README.md#noderepr)[]

Get all children nodes of this node.

###### Returns

[`NodeRepr`](README.md#noderepr)[]

###### Defined in

[index.d.ts:107](https://github.com/umijs/niddle/blob/main/index.d.ts#L107)

##### innerHtml()

> **innerHtml**(): `string`

Get the serialized html of this node, only including its all descendants.

###### Returns

`string`

###### Defined in

[index.d.ts:127](https://github.com/umijs/niddle/blob/main/index.d.ts#L127)

##### insertAfter()

> **insertAfter**(`newSibling`): `void`

Insert a new sibling after this node.

The sibling node will be remove from its previous position.

###### Parameters

| Parameter | Type |
| ------ | ------ |
| `newSibling` | [`NodeRepr`](README.md#noderepr) |

###### Returns

`void`

###### Defined in

[index.d.ts:46](https://github.com/umijs/niddle/blob/main/index.d.ts#L46)

##### insertBefore()

> **insertBefore**(`newSibling`): `void`

Insert a new sibling before this node.

The sibling node will be remove from its previous position.

###### Parameters

| Parameter | Type |
| ------ | ------ |
| `newSibling` | [`NodeRepr`](README.md#noderepr) |

###### Returns

`void`

###### Defined in

[index.d.ts:60](https://github.com/umijs/niddle/blob/main/index.d.ts#L60)

##### insertSequenceAfter()

> **insertSequenceAfter**(`newSiblings`): `void`

Insert some siblings after this node.

These sibling nodes will be remove from their previous position.

###### Parameters

| Parameter | Type |
| ------ | ------ |
| `newSiblings` | [`NodeRepr`](README.md#noderepr)[] |

###### Returns

`void`

###### Defined in

[index.d.ts:53](https://github.com/umijs/niddle/blob/main/index.d.ts#L53)

##### insertSequenceBefore()

> **insertSequenceBefore**(`newSiblings`): `void`

Insert some siblings before this node.

These sibling nodes will be remove from their previous position.

###### Parameters

| Parameter | Type |
| ------ | ------ |
| `newSiblings` | [`NodeRepr`](README.md#noderepr)[] |

###### Returns

`void`

###### Defined in

[index.d.ts:67](https://github.com/umijs/niddle/blob/main/index.d.ts#L67)

##### outerHtml()

> **outerHtml**(): `string`

Get the serialized html of this node, including its all descendants and itelf.

###### Returns

`string`

###### Defined in

[index.d.ts:122](https://github.com/umijs/niddle/blob/main/index.d.ts#L122)

##### prepend()

> **prepend**(`newChild`): `void`

Prepend a child node to this node, before existing children.

The child node will be remove from its previous position.

###### Parameters

| Parameter | Type |
| ------ | ------ |
| `newChild` | [`NodeRepr`](README.md#noderepr) |

###### Returns

`void`

###### Defined in

[index.d.ts:32](https://github.com/umijs/niddle/blob/main/index.d.ts#L32)

##### prependSequence()

> **prependSequence**(`newChildren`): `void`

Prepend some children nodes to this node by order, before existing children.

These children nodes will be remove from their previous position.

###### Parameters

| Parameter | Type |
| ------ | ------ |
| `newChildren` | [`NodeRepr`](README.md#noderepr)[] |

###### Returns

`void`

###### Defined in

[index.d.ts:39](https://github.com/umijs/niddle/blob/main/index.d.ts#L39)

##### remove()

> **remove**(): `void`

Remove a node from its parent and siblings. Children are not affected.

###### Returns

`void`

###### Defined in

[index.d.ts:72](https://github.com/umijs/niddle/blob/main/index.d.ts#L72)

##### removeAllAttributes()

> **removeAllAttributes**(): `void`

Remove all attributes of this node.

###### Returns

`void`

###### Defined in

[index.d.ts:92](https://github.com/umijs/niddle/blob/main/index.d.ts#L92)

##### removeAttribute()

> **removeAttribute**(`name`): `void`

Remove an attribute of this node by name.

###### Parameters

| Parameter | Type |
| ------ | ------ |
| `name` | `string` |

###### Returns

`void`

###### Defined in

[index.d.ts:87](https://github.com/umijs/niddle/blob/main/index.d.ts#L87)

##### select()

> **select**(`selectors`): [`NodeRepr`](README.md#noderepr)

Select the the fist node that match the given css selector, like document.querySelector.

###### Parameters

| Parameter | Type |
| ------ | ------ |
| `selectors` | `string` |

###### Returns

[`NodeRepr`](README.md#noderepr)

###### Defined in

[index.d.ts:97](https://github.com/umijs/niddle/blob/main/index.d.ts#L97)

##### selectAll()

> **selectAll**(`selectors`): [`NodeRepr`](README.md#noderepr)[]

Select all nodes that match the given css selector, like document.querySelectorAll.

###### Parameters

| Parameter | Type |
| ------ | ------ |
| `selectors` | `string` |

###### Returns

[`NodeRepr`](README.md#noderepr)[]

###### Defined in

[index.d.ts:102](https://github.com/umijs/niddle/blob/main/index.d.ts#L102)

##### setAttribute()

> **setAttribute**(`name`, `value`): `void`

Assign an attribute K-V to this node

###### Parameters

| Parameter | Type |
| ------ | ------ |
| `name` | `string` |
| `value` | `string` |

###### Returns

`void`

###### Defined in

[index.d.ts:77](https://github.com/umijs/niddle/blob/main/index.d.ts#L77)

##### setAttributes()

> **setAttributes**(`attrs`): `void`

Assign attributes K-V object to this node.

###### Parameters

| Parameter | Type |
| ------ | ------ |
| `attrs` | `Record`\<`string`, `string`\> |

###### Returns

`void`

###### Defined in

[index.d.ts:82](https://github.com/umijs/niddle/blob/main/index.d.ts#L82)

##### text()

> **text**(): `string`

Get all text nodes content of this node, including its all descendants and itelf.

###### Returns

`string`

###### Defined in

[index.d.ts:132](https://github.com/umijs/niddle/blob/main/index.d.ts#L132)

## Functions

### parse()

> **parse**(`html`): [`NodeRepr`](README.md#noderepr)

Parse string input to a html tree, return the root node.

#### Parameters

| Parameter | Type |
| ------ | ------ |
| `html` | `string` |

#### Returns

[`NodeRepr`](README.md#noderepr)

#### Defined in

[index.d.ts:10](https://github.com/umijs/niddle/blob/main/index.d.ts#L10)
