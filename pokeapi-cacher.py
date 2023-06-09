# dl pokapi pokemon until it raises a 404
import test.tests
import os, shutil
import time
import requests
import requests_cache as req_cache
import argparse
from random import randrange,uniform

# global vars
cache_location="pokeapi-cache" #files stored in this directory
pokeapi_base_url="https://pokeapi.co/api/v2/"
valid_endpoints=(
    "berry","berry-firmness", "berry-flavors",\
    "contest-type", "contest-effect", "super-contest-effect",\
    "encounter-methods", "encounter-condition", "encounter-condition-value",\
    "evolution-chain", "evolution-trigger",\
    "generation", "pokedex", "version", "version-group",\
    "item", "item-attribute", "item-category", "item-fling-effect", "item-pocket",\
    "location", "location-area", "pal-park-area", "region"\
    "machine",\
    "ability", "characteristic", "gender", "growth-rate", "nature", "pokeathlon-stat", "pokemon", "pokemon-location-area", "pokemon-color", "pokemon-form", "pokemon-habitat", "pokemon-shape", "pokemon-species", "stat", "type",\
)

# initializer argument parser for CLI
def init_parser(): # -> argparse.ArgumentParser
    parser = argparse.ArgumentParser(description="download pokeapi")
    parser.add_argument(
        '-d',
        '--dry-run',
        action="store_true",
        help="shows which URLs will be requested for debugging, will not do network requests"
    )
    parser.add_argument(
        '-b',
        '--berry',
        action="store_true",
        help="cache berry endpoint"
    )
    parser.add_argument(
        '-m',
        '--move',
        action="store_true",
        help="cache move endpoint"
    )
    parser.add_argument(
        '-p',
        '--pokemon',
        action="store_true",
        help="cache pokemon endpoint"
    )
    parser.add_argument(
        '-e',
        '--endpoint',
        action="append", # allows option to be specified multiple times, store a list and append each arg value to list e.g. -e berry -e pokemon
        help="endpoint to cache"
    )
    parser.add_argument(
        '-t',
        '--test',
        action="store_true",
        help="run tests"
    )
    parser.add_argument(
        '-l',
        '--list-endpoint',
        action="store_true",
        help="list available pokeapi endpoints"
    )
    parser.add_argument(
        '-s',
        '--store-example',
        action="store",
        help="store example"
    )
    return parser
#fed

# create the directories in the current directory if they don't exist
def mkdir(endpoint):
    if not os.path.exists(endpoint):
        os.mkdir(endpoint)
    #fi
#fed

# in: url: string, endpoint: string, session: request_cache session
def fetch(url, endpoint, session):
    # files will be saved in endpoints-cache/endpoint/id.json
    # default cache_location="pokeapi-cache"
    i=1
    while True:
        # prevent overloading the pokeapi server by waiting between requests
        wait=uniform(0.5,0.7)
        # print("wait: " + str(wait))
        time.sleep(wait)
        url2=url + "/" + str(i)
        print("GET: {_url}".format(_url=url2))
        # dry run
        if args.dry_run == True:
            pass
            if i == 100:
                break
            #fi
        # actual run
        else:
            # GET request
            r=session.get(url2)
            # path to save file to
            # you may want to change this
            file_path="{_cache_location}/{_endpoint}/{_i}.json".format(_cache_location=cache_location,_endpoint=endpoint,_i=i)
            # write to file
            with open(file_path, "w") as f:
                f.write(r.text)
            #end
            # show status code
            print("[status]: {_status_code}".format(_status_code=r.status_code))
            # quit when done
            if r.status_code == 404:
                break
            #fi
        #fi
        i=i+1
    #endloop
#fed

# in: args, list of strings from `argparse.ArgumentParser.parse_args()` to parse
def dl(args):
    endpoints = []
    
    session = req_cache.CachedSession('pokeapi-cache') # pokeapi-cache.sqlite
    # add endpoints users wants to cache,
    if args.berry   == True:
        pass
        endpoints.append("berry")
    #fi
    if args.move    == True:
        pass
        endpoints.append("move")
    #fi
    if args.pokemon == True:
        pass
        endpoints.append("pokemon")
    #fi

    # if endpoints are added multiple times it should be skipped to prevent duplication(idempotent)
    # endpoint = [a,b,c] arg=[a] -> break for a
    for arg in args.endpoint:
        # add if not duplicate
        should_append=True
        for already_present in endpoints:
            # already present exit skip adding
            if arg == already_present:
                should_append=False
                break
            #fi
        #rof
        if should_append:
            endpoints.append(arg)
            print("added " + arg)
        else:
            print("found duplicate "+arg+" skipping")
    #rof

    # run caching for each endpoint
    for endpoint in endpoints:
        pass
        print("caching endpoints for: {_endpoint}".format(_endpoint=endpoint))
        # give the function url to fetch
        url=pokeapi_base_url + endpoint
        # make folder for that particular endpoint if it doesn't exist
        mkdir("{_cache_location}".format(_cache_location=cache_location) + "/" + endpoint)
        # GET the urls and cache them to file
        fetch(url, endpoint, session)
    #end
#fed

def tests():
    print("running tests")
#fed

if __name__ == "__main__":
    # if not os.path.exists("pkmn"):
        # os.mkdir("pkmn")
    #url=pkmn_url+"/1"
    #print("attempting to fetch {_url}".format(_url=url))
    #r = requests.get(url=pkmn_url + "/0")

    #with open("pkmn/0.json", "w") as f:
        #f.write(r.text)


    parser = init_parser()
    args = parser.parse_args()
    # run tests if wanted
    if args.test == True:
        test.tests.test_duplicate_args()
        os._exit(0)
    #fi
    if args.list_endpoint:
        print("Available pokeapi endpoints:")
        print(", ".join(valid_endpoints))
        os._exit(0)
    #fi
    print("dry_run: " + str(args.dry_run))
    dl(args)
#end

# Available endpoints:
# berries
# - berry,berry-firmness, berry-flavors
# contests
# - contest-type, contest-effect, super-contest-effect
# encounters
# - encounter-methods, encounter-condition, encounter-condition-value
# evolution
# - evolution-chain, evolution-trigger
# games
# - generation, pokedex, version, version-group
# items
# - item, item-attribute, item-category, item-fling-effect, item-pocket
# locations
# - location, location-area, pal-park-area, region
# machines
# - machine
# pokemon
# - ability, characteristic, gender, growth-rate, nature, pokeathlon-stat, pokemon, pokemon-location-area, pokemon-color, pokemon-form, pokemon-habitat, pokemon-shape, pokemon-species, stat, type