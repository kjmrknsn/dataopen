module Notebook exposing (..)

import Json.Decode as Decode exposing (..)
import Html exposing (..)
import Html.Attributes exposing (..)
import Http
import Model exposing (Model)
import Msg exposing (Msg)


createNotebook: Http.Request Int
createNotebook =
    Http.post "/web/notebooks" Http.emptyBody decodeNotebook


decodeNotebook : Decode.Decoder Int
decodeNotebook  =
    Decode.at [ "id" ] Decode.int


view : Model -> Int -> List(Html Msg)
view model id =
    [ div
        [ class "row" ]
        [ div
            [ class "col" ]
            [ text "notebook" ]
        ]
    ]
