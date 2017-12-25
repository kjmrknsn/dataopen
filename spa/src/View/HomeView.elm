module View.HomeView exposing (..)

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
                    , p
                        [ class "card-text" ]
                        [ text "Data Open is a web-based collaborative data analysis platform. Users can extract, analyze and visualize data by writing Spark application code and submitting it to a Hadoop cluster. Analysis results can be shared with others as a form of a notebook." ]
                    ]
                ]
            ]
        ]
    ]
