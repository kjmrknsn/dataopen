module View.NotebookHistoryView exposing (..)

import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (onInput)
import Model
import Msg exposing (Msg(..))


header : Model.Model -> Html Msg
header model =
    nav
        [ class "bg-white px-3 py-2" ]
        [ input
            [ class "w-100 border-0 h3 m-0"
            , placeholder "Title"
            , maxlength 128
            , value model.notebookHistory.title
            , onInput UpdateNotebookHistoryTitleOnLocal
            ]
            []
        ]


view : Model.Model -> List(Html Msg)
view model =
    [ div
        [ class "row mt-3" ]
        [ div
            [ class "col" ]
            [ div
                [ class "card rounded-0" ]
                [ div
                    [ class "card-body" ]
                    []
                ]
            ]
        ]
    ]
