module Model exposing (..)

import NotebookHistory
import Page exposing (Page(..))
import Paragraph


type alias Model =
    { page : Page
    , uid : String
    , notebookHistory : NotebookHistory.Model
    , paragraphs : List(Paragraph.Model)
    }


new : Model
new =
    { page = Home
    , uid = ""
    , notebookHistory = NotebookHistory.new
    , paragraphs = []
    }


updatePage : Model -> Page -> Model
updatePage model page =
    { model | page = page, notebookHistory = NotebookHistory.new, paragraphs = [] }


updateUid : Model -> String -> Model
updateUid model uid =
    { model | uid = uid }


updateNotebookHistory : Model -> NotebookHistory.Model -> Model
updateNotebookHistory model notebookHistory =
    { model | notebookHistory = notebookHistory }


updateParagraphs : Model -> List(Paragraph.Model) -> Model
updateParagraphs model paragraphs =
    { model | paragraphs = paragraphs }
