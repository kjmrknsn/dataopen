module View.NotebookHistoryView exposing (..)

import Html exposing (..)
import Html.Attributes exposing (..)
import Model
import Msg exposing (Msg(..))


header : Model.Model -> Html Msg
header model =
    nav
        [ class "bg-white" ]
        [ input
            []
            []
        ]


view : Model.Model -> List(Html Msg)
view model =
    [ div
        []
        [ text "hi" ]
    ]
