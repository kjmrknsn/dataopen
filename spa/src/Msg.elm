module Msg exposing (..)

import Http
import Navigation exposing (Location)
import NotebookHistory

type Msg
    = UrlChange Location
    | GetUidResult (Result Http.Error String)
    | CreateNotebook
    | CreateNotebookResult (Result Http.Error Int)
    | CreateNotebookHistoryResult (Result Http.Error NotebookHistory.Model)
