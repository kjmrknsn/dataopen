module Msg exposing (..)

import Json.Decode as Decode
import Http
import Navigation exposing (Location)
import NotebookHistory


type Msg
    = UrlChange Location
    | GetUidResult (Result Http.Error String)
    | CreateNotebook
    | CreateNotebookResult (Result Http.Error Int)
    | CreateNotebookHistoryResult (Result Http.Error NotebookHistory.Model)
    | GetNotebookHistoryResult (Result Http.Error NotebookHistory.Model)
    | UpdateNotebookHistoryTitleOnLocal String
    | UpdateNotebookHistoryTitle
    | UpdateNotebookHistoryTitleResult (Result Http.Error Decode.Value)
    | CompleteNotebookHistory
    | CompleteNotebookHistoryResult (Result Http.Error Decode.Value)
