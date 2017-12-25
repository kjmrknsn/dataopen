module Main exposing (..)

import Html exposing (..)
import Http
import Model exposing (Model)
import Msg exposing (Msg(..))
import Navigation exposing (Location)
import Notebook exposing (createNotebook, decodeNotebook)
import NotebookHistory exposing ( getNotebookHistory
                                , createNotebookHistory
                                , decodeNotebookHistory )
import Page exposing (Page(..))
import Uid exposing (..)
import UrlParser exposing ((</>))
import View.MainContentView as MainContentView
import View.NavbarView as NavbarView


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
    let
        (model, urlCmd) = urlUpdate Model.new location
        getUidCmd = Http.send GetUidResult getUid
    in
        (model, Cmd.batch [ urlCmd, getUidCmd ])


update : Msg -> Model -> (Model, Cmd Msg)
update msg model =
    case msg of
        UrlChange location ->
            urlUpdate model location
        GetUidResult result ->
            case result of
                Ok uid ->
                    (Model.updateUid model uid, Cmd.none)
                Err _ ->
                    error model
        CreateNotebook ->
            (model, Http.send CreateNotebookResult createNotebook)
        CreateNotebookResult result ->
            case result of
                Ok id ->
                    (model, Http.send CreateNotebookHistoryResult (createNotebookHistory id))
                Err _ ->
                    error model
        CreateNotebookHistoryResult result ->
            case result of
                Ok notebookHistory ->
                    ( model
                    , Navigation.load (
                        "#/notebooks/"
                      ++ (toString notebookHistory.notebookId)
                      ++ "/notebook_histories/"
                      ++ (toString notebookHistory.id)
                      ++ "/edit"
                      )
                    )
                Err _ ->
                    error model
        GetNotebookHistoryResult result ->
            case result of
                Ok notebookHistory ->
                    (Model.updateNotebookHistory model (Just notebookHistory), Cmd.none)
                Err _ ->
                    error model


urlUpdate : Model -> Navigation.Location -> ( Model, Cmd Msg )
urlUpdate model location =
    case decode location of
        Nothing ->
            ( Model.updatePage model NotFound, Cmd.none )
        Just (EditNotebookHistory notebookId id) ->
             ( Model.updatePage model (EditNotebookHistory notebookId id)
             , Http.send GetNotebookHistoryResult (getNotebookHistory notebookId id) )
        Just page ->
            ( Model.updatePage model page, Cmd.none )


decode : Location -> Maybe Page
decode location =
    UrlParser.parseHash routeParser location


routeParser : UrlParser.Parser (Page -> a) a
routeParser =
    UrlParser.oneOf
        [ UrlParser.map Home UrlParser.top
        , UrlParser.map EditNotebookHistory
            ( UrlParser.s "notebooks"
            </> UrlParser.int
            </> UrlParser.s "notebook_histories"
            </> UrlParser.int
            </> UrlParser.s "edit"
            )
        , UrlParser.map Error (UrlParser.s "error")
        ]


error : Model -> (Model, Cmd Msg)
error model =
    (model, Navigation.load "#/error")


view : Model -> Html Msg
view model =
    div
        [] <|
        List.append
            [ NavbarView.view model ]
            (MainContentView.view model)
