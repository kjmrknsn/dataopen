module Navbar exposing (..)

import ExtAttributes exposing (..)
import Html exposing (..)
import Html.Attributes exposing (..)
import Model exposing (Model)
import Msg exposing (Msg(..))
import Version exposing (..)


view : Model -> Html Msg
view model =
    nav
        [ class "navbar navbar-expand-sm navbar-dark bg-dark"
        ]
        [ div
            []
            [ a
                [ class "navbar-brand"
                , href "#"
                ]
                [ text "Data Open"
                ]
            , span
                [ class "small text-secondary"
                ]
                [ text ("version " ++ version)
                ]
            ]
        , button
            [ class "navbar-toggler"
            , type_ "button"
            , dataToggle "collapse"
            , dataTarget "#navbarSupportedContent"
            , ariaControls "navbarSupportedContent"
            , ariaExpanded "false"
            , ariaLabel "Toggle navigation"
            ]
            [ span [ class "navbar-toggler-icon" ] []
            ]
        , div
            [ class "collapse navbar-collapse"
            , id "navbarSupportedContent"
            ]
            []
        ]
