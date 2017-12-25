module Main exposing (..)

import Home exposing (..)
import Html exposing (..)
import Html.Attributes exposing (..)
import Http
import Model exposing (Model)
import Msg exposing (Msg(..))
import Navbar
import Navigation exposing (Location)
import Notebook exposing (..)
import NotFound as NotFound_
import Page exposing (Page(..))
import Uid exposing (..)
import UrlParser exposing ((</>))


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
                    (model, Cmd.none)
        CreateNotebook ->
            (model, Http.send CreateNotebookResult createNotebook)
        CreateNotebookResult result ->
            case result of
                Ok notebook_id ->
                    (model, Navigation.load ("#/notebooks/" ++ (toString notebook_id) ++ "/edit"))
                Err _ ->
                    (model, Cmd.none)


urlUpdate : Model -> Navigation.Location -> ( Model, Cmd Msg )
urlUpdate model location =
    case decode location of
        Nothing ->
            ( Model.updatePage model NotFound, Cmd.none )

        Just page ->
            ( Model.updatePage model page, Cmd.none )


decode : Location -> Maybe Page
decode location =
    UrlParser.parseHash routeParser location


routeParser : UrlParser.Parser (Page -> a) a
routeParser =
    UrlParser.oneOf
        [ UrlParser.map Home UrlParser.top
        ]


view : Model -> Html Msg
view model =
    div
        []
        [ Navbar.view model
        , mainContent model
        ]


mainContent : Model -> Html Msg
mainContent model =
    main_
        []
        [ div
            [ class "container-fluid" ] <|
                case model.page of
                    Home ->
                        Home.view model
                    NotFound ->
                        NotFound_.view model
        ]
