module View.ParagraphView exposing (..)

import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (onBlur, onClick, onInput)
import Model
import Msg exposing (Msg(..))
import Paragraph


view : Model.Model -> Paragraph.Model -> Html Msg
view model paragraph =
    div
        [ class "row mt-3" ]
        [ div
            [ class "col" ]
            [ div
                [ class "card rounded-0" ]
                [ div
                    [ class "card-body" ]
                    [ textarea
                        [ id ("paragraph-" ++ toString paragraph.id)
                        ]
                        [ text paragraph.code ]
                    ]
                ]
            ]
        ]

