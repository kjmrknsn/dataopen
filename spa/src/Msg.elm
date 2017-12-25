module Msg exposing (..)

import Http
import Navigation exposing (Location)


type Msg
    = UrlChange Location
    | GetUidResult (Result Http.Error String)
    | CreateNotebook
    | CreateNotebookResult (Result Http.Error Int)
