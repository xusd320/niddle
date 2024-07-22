**niddle** • **Docs**

***

# niddle

## Classes

### NodeRepr

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

[index.d.ts:14](https://github.com/umijs/niddle/blob/main/index.d.ts#L14)

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

[index.d.ts:20](https://github.com/umijs/niddle/blob/main/index.d.ts#L20)

##### clone()

> **clone**(): [`NodeRepr`](README.md#noderepr)

Clone this node to a new instance, not clone its descendants.

###### Returns

[`NodeRepr`](README.md#noderepr)

###### Defined in

[index.d.ts:90](https://github.com/umijs/niddle/blob/main/index.d.ts#L90)

##### cloneRecursive()

> **cloneRecursive**(): [`NodeRepr`](README.md#noderepr)

Clone this node to a new instance, including its all descendants.

###### Returns

[`NodeRepr`](README.md#noderepr)

###### Defined in

[index.d.ts:92](https://github.com/umijs/niddle/blob/main/index.d.ts#L92)

##### constructor()

> **constructor**(): `void`

The node object, cann't be instantiated in javascript. So call the constructor will throw an error

###### Returns

`void`

###### Defined in

[index.d.ts:96](https://github.com/umijs/niddle/blob/main/index.d.ts#L96)

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

[index.d.ts:74](https://github.com/umijs/niddle/blob/main/index.d.ts#L74)

##### getAttributes()

> **getAttributes**(): `Record`\<`string`, `string`\>

Get attributes K-V object of this node.

###### Returns

`Record`\<`string`, `string`\>

###### Defined in

[index.d.ts:76](https://github.com/umijs/niddle/blob/main/index.d.ts#L76)

##### getChildren()

> **getChildren**(): [`NodeRepr`](README.md#noderepr)[]

Get all children nodes of this node.

###### Returns

[`NodeRepr`](README.md#noderepr)[]

###### Defined in

[index.d.ts:72](https://github.com/umijs/niddle/blob/main/index.d.ts#L72)

##### innerHtml()

> **innerHtml**(): `string`

Get the serialized html of this node, only including its all descendants;.

###### Returns

`string`

###### Defined in

[index.d.ts:80](https://github.com/umijs/niddle/blob/main/index.d.ts#L80)

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

[index.d.ts:38](https://github.com/umijs/niddle/blob/main/index.d.ts#L38)

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

[index.d.ts:50](https://github.com/umijs/niddle/blob/main/index.d.ts#L50)

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

[index.d.ts:44](https://github.com/umijs/niddle/blob/main/index.d.ts#L44)

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

[index.d.ts:56](https://github.com/umijs/niddle/blob/main/index.d.ts#L56)

##### outerHtml()

> **outerHtml**(): `string`

Get the serialized html of this node, including its all descendants and itelf;.

###### Returns

`string`

###### Defined in

[index.d.ts:78](https://github.com/umijs/niddle/blob/main/index.d.ts#L78)

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

[index.d.ts:26](https://github.com/umijs/niddle/blob/main/index.d.ts#L26)

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

[index.d.ts:32](https://github.com/umijs/niddle/blob/main/index.d.ts#L32)

##### remove()

> **remove**(): `void`

Remove a node from its parent and siblings. Children are not affected.

###### Returns

`void`

###### Defined in

[index.d.ts:58](https://github.com/umijs/niddle/blob/main/index.d.ts#L58)

##### removeAllAttributes()

> **removeAllAttributes**(): `void`

Remove all attributes of this node

###### Returns

`void`

###### Defined in

[index.d.ts:66](https://github.com/umijs/niddle/blob/main/index.d.ts#L66)

##### removeAttribute()

> **removeAttribute**(`name`): `void`

Remove an attribute of this node by name

###### Parameters

| Parameter | Type |
| ------ | ------ |
| `name` | `string` |

###### Returns

`void`

###### Defined in

[index.d.ts:64](https://github.com/umijs/niddle/blob/main/index.d.ts#L64)

##### select()

###### select(selectors)

> **select**(`selectors`): [`NodeRepr`](README.md#noderepr)

Select the the fist node that match the given css selector, like document.querySelector.

###### Parameters

| Parameter | Type |
| ------ | ------ |
| `selectors` | `string` |

###### Returns

[`NodeRepr`](README.md#noderepr)

###### Defined in

[index.d.ts:68](https://github.com/umijs/niddle/blob/main/index.d.ts#L68)

###### select(selectors)

> **select**(`selectors`): [`NodeRepr`](README.md#noderepr)

###### Parameters

| Parameter | Type |
| ------ | ------ |
| `selectors` | `"html"` |

###### Returns

[`NodeRepr`](README.md#noderepr)

###### Defined in

[index.d.ts:97](https://github.com/umijs/niddle/blob/main/index.d.ts#L97)

###### select(selectors)

> **select**(`selectors`): [`NodeRepr`](README.md#noderepr)

###### Parameters

| Parameter | Type |
| ------ | ------ |
| `selectors` | `"head"` |

###### Returns

[`NodeRepr`](README.md#noderepr)

###### Defined in

[index.d.ts:98](https://github.com/umijs/niddle/blob/main/index.d.ts#L98)

###### select(selectors)

> **select**(`selectors`): [`NodeRepr`](README.md#noderepr)

###### Parameters

| Parameter | Type |
| ------ | ------ |
| `selectors` | `"body"` |

###### Returns

[`NodeRepr`](README.md#noderepr)

###### Defined in

[index.d.ts:99](https://github.com/umijs/niddle/blob/main/index.d.ts#L99)

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

[index.d.ts:70](https://github.com/umijs/niddle/blob/main/index.d.ts#L70)

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

[index.d.ts:60](https://github.com/umijs/niddle/blob/main/index.d.ts#L60)

##### setAttributes()

> **setAttributes**(`attrs`): `void`

Assign attributes K-V object to this node

###### Parameters

| Parameter | Type |
| ------ | ------ |
| `attrs` | `Record`\<`string`, `string`\> |

###### Returns

`void`

###### Defined in

[index.d.ts:62](https://github.com/umijs/niddle/blob/main/index.d.ts#L62)

##### text()

> **text**(): `string`

Get all text nodes content of this node, including its all descendants and itelf;.

###### Returns

`string`

###### Defined in

[index.d.ts:82](https://github.com/umijs/niddle/blob/main/index.d.ts#L82)

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

[index.d.ts:7](https://github.com/umijs/niddle/blob/main/index.d.ts#L7)
