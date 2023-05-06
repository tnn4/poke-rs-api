# dl pokapi pokemon until it raises a 404
import os, shutil
import time
import requests
import requests_cache as req_cache
import argparse
from random import randrange,uniform

pokeapi_base_url="https://pokeapi.co/api/v2/"
pkmn_url = pokeapi_base_url + "pokemon"

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

    return parser
#fed

# create the directories in the current directory if they don't exist
def mkdir(option):
    if not os.path.exists(option):
        os.mkdir(option)
    #fi
#fed

# in: url: string, option: string, session: request_cache session
def fetch(url, option, session):
    
    i=1
    while True:
        wait=uniform(0.5,1.0)
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
        else:
            # GET request
            r=session.get(url2)
            
            # write to file
            with open("{_option}/{_i}.json".format(_option=option,_i=i), "w") as f:
                f.write(r.text)
            #end
            # next
            
            
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
    options_list = []
    
    session = req_cache.CachedSession('pokeapi-pkmn-cache')
    # create array from parsed args
    if args.berry   == True:
        pass
        options_list.append("berry")
    #fi
    if args.move    == True:
        pass
        options_list.append("move")
    #fi
    if args.pokemon == True:
        pass
        options_list.append("pokemon")
    #fi

    for option in options_list:
        pass
        print("caching endpoints for: {_option}".format(_option=option))
        # give the function url to fetch
        url=pokeapi_base_url + option
        # make folder
        mkdir(option)
        # GET the urls
        fetch(url, option, session)
    #end
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
    print("dry_run: " + str(args.dry_run))
    dl(args)
#end

    # endpoints:
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