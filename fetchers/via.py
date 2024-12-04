import gtfsrt_pb2 as gtfsrt_pb2
import requests
from google.protobuf import json_format
import json
import ast

import pprint as PP

pp = PP.PrettyPrinter(indent=2)
pprint = pp.pprint

def rest_status_color_helper(code: int) -> str:
    """
    Changes text color in supported terminals based on status code.
    200-299: Success, green
    300-399: Unsure,  yellow
    400+   : Failed,  red
    """
    ansi_esc: int = 32 if code < 300 else 33 if code < 400 else 31
    return f'\x1b[{ansi_esc}m{code}\x1b[0m'


def write_to_file(path: str, content: str) -> None:
    """
    Writes the content string to the specified path
    """
    with open(path, 'w') as f:
        f.write(content)


def main(endpoint):
    response: requests.Response = requests.get(endpoint)   
    bytestream, rest_status = response.content, response.status_code  
    print(f"""{endpoint} returned status {rest_status} with length {len(bytestream) / 1000} KB""") 
    feedmsg = gtfsrt_pb2.FeedMessage()  # Create an empty instance of a FeedMessage (class that holds all GTFS-RT data)
    proto_status1 = feedmsg.ParseFromString(bytestream)
    print(f"\x1b[33mStatusCode from \x1b[36mfeedmsg.ParseFromString(...) \x1b[0m: \x1b[34m{proto_status1}\x1b[0m")
    return feedmsg


def collect():
    bus = main("http://gtfs.viainfo.net/vehicle/vehiclepositions.pb")
    bustrip = main("http://gtfs.viainfo.net/tripupdate/tripupdates.pb")

    
    fm_str = json_format.MessageToJson(bus)
    fm_str2 = json_format.MessageToJson(bustrip)


    write_to_file("data/via/vehicles-bus.txt", fm_str)
    write_to_file("data/via/trips-bus.txt", fm_str2)


collect()
