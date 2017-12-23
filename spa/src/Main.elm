module Main exposing (..)

import Html exposing (..)
import Html.Attributes exposing (..)
import Http
import Model exposing (Model)
import Msg exposing (Msg(..))
import Navbar
import Navigation exposing (Location)
import Notebook exposing (..)
import Uid exposing (..)


main : Program Never Model Msg
main =
    Navigation.program UrlChange
        { init = init
        , update = update
        , view = view
        , subscriptions = (\_ -> Sub.none)
        }


init : Location -> (Model, Cmd Msg)
init location =
    ( Model.new
    , Http.send GetUidResult getUid
    )


update : Msg -> Model -> (Model, Cmd Msg)
update msg model =
    case msg of
        UrlChange location ->
            (model, Cmd.none)
        GetUidResult result ->
            case result of
                Ok uid ->
                    (Model.updateUid model uid, Cmd.none)
                Err _ ->
                    (model, Cmd.none)
        CreateNotebook ->
            (model, Http.send CreateNotebookResult createNotebook)
        CreateNotebookResult result ->
            case result of
                Ok notebook_id ->
                    (model, Cmd.none)
                Err _ ->
                    (model, Cmd.none)



view : Model -> Html Msg
view model =
    div
        [ class "h-100"
        ]
        [ Navbar.view model
        ]
