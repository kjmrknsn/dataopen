module Model exposing (..)

import NotebookHistory
import Page exposing (Page(..))


type alias Model =
    { page : Page
    , uid : String
    , notebookHistory : NotebookHistory.Model
    }


new : Model
new =
    { page = Home
    , uid = ""
    , notebookHistory = NotebookHistory.new
    }


updatePage : Model -> Page -> Model
updatePage model page =
    { model | page = page, notebookHistory = NotebookHistory.new }


updateUid : Model -> String -> Model
updateUid model uid =
    { model | uid = uid }


updateNotebookHistory : Model -> NotebookHistory.Model -> Model
updateNotebookHistory model notebookHistory =
    { model | notebookHistory = notebookHistory }
