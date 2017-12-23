module ExtAttributes exposing (..)

import Html exposing (..)
import Html.Attributes exposing (..)


ariaControls : (String -> Attribute msg)
ariaControls =
    attribute "aria-controls"


ariaExpanded : (String -> Attribute msg)
ariaExpanded =
    attribute "aria-expanded"


ariaHaspopup : (String -> Attribute msg)
ariaHaspopup =
    attribute "aria-haspopup"


ariaLabel : (String -> Attribute msg)
ariaLabel =
    attribute "aria-label"


dataTarget : (String -> Attribute msg)
dataTarget =
    attribute "data-target"


dataToggle : (String -> Attribute msg)
dataToggle =
    attribute "data-toggle"
