# Properties

Here is a list of all currently supported properties. Note that these are properties which are provived by Bevy ECSS but you can also add your own properties at anytime.

_Before reading properties description, we'll use this notation to describe accepted values:_

|        Notation        | Description                                                                                                                                                                                                                            |
| :--------------------: | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
|        `00.00%`        | Any percent value, like `93%` or `4.45%`                                                                                                                                                                                               |
|       `00.00px`        | Any dimensional value, like `11px` or `0.99px`                                                                                                                                                                                         |
|        `00.00`         | Any number value, like `0` or `14.2`                                                                                                                                                                                                   |
| `<ident>` \| `<ident>` | Only one of the identifiers are allowed, without quotes, like `none` or `hidden`                                                                                                                                                       |
|  <`area-short-hand`>   | Allows the [`short hand area constructor`](https://developer.mozilla.org/en-US/docs/Web/CSS/margin#syntax) by using either dimensions or percentage, like `10px` or `5% 10px 3% auto`. No global values are supported yet |


### [`Style`](https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html) properties

|       Property        |                                                                            Values                                                                             | Description                                                                                                                                                                                                                                                               |
|:---------------------:|:-------------------------------------------------------------------------------------------------------------------------------------------------------------:| :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
|       `display`       |                                                                       `flex` \| `none`                                                                        | Applies the  `display`         property on [`display`](https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html#structfield.display) field of all sections on matched [`Style`](https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html) components.                 |
|    `position-type`    |                                                                   `absolute` \| `relative`                                                                    | Applies the  `position-type`   property on [`position_type`](https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html#structfield.position_type) field of all sections on matched [`Style`](https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html) components.     |
|      `direction`      |                                                        `inherit` \| `left-to-right` \| `right-to-left`                                                        | Applies the  `direction`       property on [`direction`](https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html#structfield.direction) field of all sections on matched [`Style`](https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html) components.             |
|   `flex-direction`    |                                                    `row` \| `column` \| `row-reverse` \| `column-reverse`                                                     | Applies the  `flex-direction`  property on [`flex_direction`](https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html#structfield.flex_direction) field of all sections on matched [`Style`](https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html) components.   |
|      `flex-wrap`      |                                                             `no-wrap` \| `wrap` \| `wrap-reverse`                                                             | Applies the  `flex-wrap`       property on [`flex_wrap`](https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html#structfield.flex_wrap) field of all sections on matched [`Style`](https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html) components.             |
|     `align-items`     |                                               `flex-start` \| `flex-end` \| `center` \| `baseline` \| `stretch`                                               | Applies the  `align-items`     property on [`align_items`](https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html#structfield.align_items) field of all sections on matched [`Style`](https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html) components.         |
|     `align-self`      |                                          `auto` \| `flex-start` \| `flex-end` \| `center` \| `baseline` \| `stretch`                                          | Applies the  `align-self`      property on [`align_self`](https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html#structfield.align_self) field of all sections on matched [`Style`](https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html) components.           |
|    `align-content`    |                                   `flex-start` \| `flex-end` \| `center` \| `stretch` \| `space-between` \| `space-around`                                    | Applies the  `align-content`   property on [`align_content`](https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html#structfield.align_content) field of all sections on matched [`Style`](https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html) components.     |
|   `justify-content`   |                                 `flex-start` \| `flex-end` \| `center` \| `space-between` \| `space-around` \| `space-evenly`                                 | Applies the  `justify-content` property on [`justify_content`](https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html#structfield.justify_content) field of all sections on matched [`Style`](https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html) components. |
|     `overflow-x`      |                                                                     `visible` \| `hidden`                                                                     | Applies the  `overflow-x`      property on [`overflow.x`](https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html#structfield.overflow) field of all sections on matched [`Style`](https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html) components.                |
|     `overflow-y`      |                                                                     `visible` \| `hidden`                                                                     | Applies the  `overflow-y`      property on [`overflow.y`](https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html#structfield.overflow) field of all sections on matched [`Style`](https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html) components.                |
|        `left`         |                                                                     `00.00%` \| `00.00px`                                                                     | Applies the             property on [`left`](https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html#structfield.left) field of all matched components.                                                                                                   |
|        `right`        |                                                                     `00.00%` \| `00.00px`                                                                     | Applies the             property on [`right`](https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html#structfield.right) field of all matched components.                                                                                                  |
|         `top`         |                                                                     `00.00%` \| `00.00px`                                                                     | Applies the             property on [`top`](https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html#structfield.top) field of all matched components.                                                                                                    |
|       `bottom`        |                                                                     `00.00%` \| `00.00px`                                                                     | Applies the             property on [`bottom`](https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html#structfield.bottom) field of all matched components.                                                                                                 |
|        `width`        |                                                                     `00.00%` \| `00.00px`                                                                     | Applies the             property on [`width`](https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html#structfield.width) field of all matched components.                                                                                                          |
|       `height`        |                                                                     `00.00%` \| `00.00px`                                                                     | Applies the             property on [`height`](https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html#structfield.height) field of all matched components.                                                                                                         |
|      `min-width`      |                                                                     `00.00%` \| `00.00px`                                                                     | Applies the             property on [`min_width`](https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html#structfield.min_width) field of all matched components.                                                                                                  |
|     `min-height`      |                                                                     `00.00%` \| `00.00px`                                                                     | Applies the             property on [`min_height`](https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html#structfield.min_height) field of all matched components.                                                                                                 |
|      `max-width`      |                                                                     `00.00%` \| `00.00px`                                                                     | Applies the             property on [`max_width`](https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html#structfield.max_width) field of all matched components.                                                                                                  |
|     `max-height`      |                                                                     `00.00%` \| `00.00px`                                                                     | Applies the             property on [`max_height`](https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html#structfield.max_height) field of all matched components.                                                                                                 |
|     `flex-basis`      |                                                                     `00.00%` \| `00.00px`                                                                     | Applies the             property on [`flex_basis`](https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html#structfield.flex_basis) field of all matched components.                                                                                                 |
|      `flex-grow`      |                                                                       `0` \| `1` \| `2`                                                                       | Applies the             property on [`flex_grow`](https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html#structfield.flex_grow)   field of all matched components.                                                                                                    |
|     `flex-shrink`     |                                                                       `0` \| `1` \| `2`                                                                       | Applies the             property on [`flex_shrink`](https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html#structfield.flex_shrink) field of all matched components.                                                                                                  |
|    `aspect-ratio`     |                                                                       `00.00` \| `none`                                                                       | Applies the             property on [`aspect_ratio`](https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html#structfield.aspect_ratio) field of all matched components.                                                                                                |
|       `margin`        |                                                                      <`area-short-hand`>                                                                      | Applies the             property on [`margin`](https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html#structfield.margin) field of all matched components.                                                                                                            |
|       `padding`       |                                                                      <`area-short-hand`>                                                                      | Applies the             property on [`padding`](https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html#structfield.padding) field of all matched components.                                                                                                          |
|       `border`        |                                                                      <`area-short-hand`>                                                                      | Applies the             property on [`border`](https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html#structfield.border) field of all matched components.                                                                                                            |

### [`Text`](https://docs.rs/bevy/latest/bevy/prelude/struct.Text.html) properties

|     Property     |                                                                            Values                                                                            | Description                                                                                                                                                                                                                             |
| :--------------: | :----------------------------------------------------------------------------------------------------------------------------------------------------------: | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
|     `color`      | [`named-colors`](https://developer.mozilla.org/en-US/docs/Web/CSS/named-color) \| [`hex_colors`](https://developer.mozilla.org/en-US/docs/Web/CSS/hex-color) | Applies the property on [`style.color`](https://docs.rs/bevy/latest/bevy/text/struct.TextSection.html#structfield.style) for all [`sections`](https://docs.rs/bevy/latest/bevy/text/struct.TextSection.html) of matched components.     |
|                  |
|      `font`      |                                                                     `"path/to/font.ttf"`                                                                     | Applies the property on [`style.font`](https://docs.rs/bevy/latest/bevy/text/struct.TextSection.html#structfield.style) for all [`sections`](https://docs.rs/bevy/latest/bevy/text/struct.TextSection.html) of matched components.      |
|                  |
|   `font-size`    |                                                                           `00.00`                                                                            | Applies the property on [`style.font_size`](https://docs.rs/bevy/latest/bevy/text/struct.TextSection.html#structfield.style) for all [`sections`](https://docs.rs/bevy/latest/bevy/text/struct.TextSection.html) of matched components. |
|                  |
|  `text-content`  |                                                                     `"Some text value"`                                                                      | Applies the property on [`value`](https://docs.rs/bevy/latest/bevy/text/struct.TextSection.html#structfield.value) for all [`sections`](https://docs.rs/bevy/latest/bevy/text/struct.TextSection.html) of matched components.           |
|                  |
|   `text-align`   |                                                                `left` \| `center` \| `right`                                                                 | Applies the property on [`alignment`](https://docs.rs/bevy/latest/bevy/text/struct.Text.html#structfield.alignment) of all matched components.                                                                                          |
|                  |

### Components properties

|       Property       |                                                                            Values                                                                            | Description                                                                                                                                  |
|:--------------------:| :----------------------------------------------------------------------------------------------------------------------------------------------------------: |:---------------------------------------------------------------------------------------------------------------------------------------------|
|  `background-color`  | [`named-colors`](https://developer.mozilla.org/en-US/docs/Web/CSS/named-color) \| [`hex_colors`](https://developer.mozilla.org/en-US/docs/Web/CSS/hex-color) | Applies the property on [`BackgroundColor`](https://docs.rs/bevy/latest/bevy/prelude/struct.BackgroundColor.html) of all matched components. |
|    `border-color`    | [`named-colors`](https://developer.mozilla.org/en-US/docs/Web/CSS/named-color) \| [`hex_colors`](https://developer.mozilla.org/en-US/docs/Web/CSS/hex-color)  | Applies the property on [`BorderColor`](https://docs.rs/bevy/latest/bevy/prelude/struct.BorderColor.html) of all matched components.         |                                                                                                         |

### Image properties

|   Property   |       Values       | Description                                                                                                                                                                                                                          |
|:------------:|:------------------:|:-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `image-path` | "path/to/image.png" | Applies the property on [`image.texture`](https://docs.rs/bevy/latest/bevy/prelude/struct.UiImage.html#structfield.texture) for all [`images`](https://docs.rs/bevy/latest/bevy/ui/struct.UiImage.html) of matched components. |