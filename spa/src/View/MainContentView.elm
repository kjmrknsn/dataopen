module View.MainContentView exposing (..)

import Html exposing (..)
import Html.Attributes exposing (..)
import Model exposing (Model)
import Msg exposing (Msg)
import Page exposing (Page(..))
import View.HomeView as HomeView
import View.NotebookHistoryView as NotebookHistoryView
import View.NotFoundView as NotFoundView


view : Model -> Html Msg
view model =
    main_
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
        ]
