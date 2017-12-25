module View.MainContentView exposing (..)

import Html exposing (..)
import Html.Attributes exposing (..)
import Model exposing (Model)
import Msg exposing (Msg)
import Page exposing (Page(..))
import View.ErrorView as ErrorView
import View.HomeView as HomeView
import View.NotebookHistoryView as NotebookHistoryView
import View.NotFoundView as NotFoundView


view : Model -> List(Html Msg)
view model =
    [
        case model.page of
            EditNotebookHistory _ _ ->
                NotebookHistoryView.header model
            _ ->
                div [] []
    ,   main_
            []
            [ div
                [ class "container-fluid" ] <|
                    case model.page of
                        Home ->
                            HomeView.view model
                        EditNotebookHistory _ _ ->
                            NotebookHistoryView.view model
                        NotFound ->
                            NotFoundView.view model
                        Error ->
                            ErrorView.view model
            ]
    ]
