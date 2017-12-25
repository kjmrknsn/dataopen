module Home exposing (..)

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
            [ div
                [ class "card rounded-0" ]
                [ div
                    [ class "card-body" ]
                    [ h4
                        [ class "card-title" ]
                        [ text "Data Open" ]
                    ]
                ]
            ]
        ]
    ]
