module View.NotebookHistoryView exposing (..)

import Html exposing (..)
import Html.Attributes exposing (..)
import Model
import Msg exposing (Msg(..))


view : Model.Model -> List(Html Msg)
view model =
    [ div
        []
        [ text "Hello Notebook History" ]
    ]
