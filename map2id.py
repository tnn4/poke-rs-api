import sys, os
import json

# read pokeapi-cache to create a hashmap to speed up looks up by the backend
# the endpoint files need to be all cached before this can work

# see: https://pynative.com/python-count-number-of-files-in-a-directory/
# returns number of files in a directory
def get_file_count(dir_path):
    count=0
    if os.path.exists(dir_path):
        for path in os.listdir(dir_path):
            if os.path.isfile(os.path.join(dir_path, path)):
                count += 1
            #fi
        #rof
    else:
        str="{_dir_path} not found".format(_dir_path=dir_path)
        raise Exception(str)
    #fi
    return count
#fed

if __name__ == "__main__":
    cache_root="pokeapi-cache"
    endpoint_arg="pokemon"
    dir_path="PATH_ERROR"
    valid_arg=False
    valid_endpoints=("berry","move","pokemon")
    # no arg
    if len(sys.argv) == 1:
        print("Requires 1 argument: endpoint to map e.g. <berry|move|pokemon>")
        os._exit(-1)
    # help
    elif sys.argv[1] == "-h" or sys.argv[1] == "--help":
        print("Map endpoint item name to id")
        print("This information will be used by the backend to speed up lookups for item names")
        print("Requires 1 argument: endpoint to map <berry|move|pokemon>")
        print("e.g. `python map2id.py berry` - creates a mapping for pokeapi berry endpoint at file berry2id.toml")
        os._exit(-1)
    # arg valid
    else:
        print("argv= "+ str(len(sys.argv)))
        print("argv[1]="+sys.argv[1])
        endpoint_arg = sys.argv[1]
        # look for valid endpoint argument
        for e in valid_endpoints:
            if e == endpoint_arg.lower():
                print("endpoint found")
                valid_arg=True
        #rof

        #fi

    #fi
    print("valid_arg "+ str(valid_arg))
    if valid_arg:
        if os.path.exists(cache_root + "/" + endpoint_arg):
            dir_path=cache_root+"/"+endpoint_arg
            print("dir_path=" + dir_path)
        #fi
    else:
        os._exit(-1)
    
    with open("mappings/{_endpoint}2id.toml".format(_endpoint=endpoint_arg), "w") as toml:
        end=get_file_count(dir_path) + 1
        for n in range(1, end):
            with open("{_dir_path}/{_n}.json".format(_dir_path=dir_path,_n=n), "r") as f:
                data = json.load(f)
                line=data["name"]+"="+str(data["id"])+"\n"
                print(line)
                toml.write(line)
                f.close()
            #end
        #rof
    #close
#fi