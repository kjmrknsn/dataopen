module View.NotFoundView exposing (..)

import Html exposing (..)
import Html.Attributes exposing (..)
import Model exposing (Model)
import Msg exposing (Msg)


view : Model -> List(Html Msg)
view model =
    [ div
        [ class "row mt-3" ]
        [ div
            [ class "col" ]
            [ h1
                [ class "text-center" ]
                [ text "404 Not Found" ]
            ]
        ]
    ]
