module Main exposing (..)

import Html exposing (..)
import Html.Attributes exposing (..)
import Model exposing (Model)
import Msg exposing (Msg(..))
import Navbar
import Navigation exposing (Location)


main =
    Navigation.program UrlChange
        { init = init
        , update = update
        , view = view
        , subscriptions = (\_ -> Sub.none)
        }


init : Location -> (Model, Cmd Msg)
init location =
    (Model.new, Cmd.none)


update : Msg -> Model -> (Model, Cmd Msg)
update msg model =
    case msg of
        UrlChange location ->
            (model, Cmd.none)


view : Model -> Html Msg
view model =
    div
        [ class "h-100"
        ]
        [ Navbar.view model
        ]
