import json

if __name__ == "__main__":
    with open("poke2id.toml", "w") as toml:
        for n in range(1, 1012):
            with open("pkmn/{_n}.json".format(_n=n), "r") as f:
                data = json.load(f)
                line=data["name"]+"="+str(data["id"])+"\n"
                print(line)
                toml.write(line)
                f.close()
            #end
        #rof
    #nepo

    #f = open("pkmn/1.json")
    #json_dump = json.dumps(f)
    #print(json_dump)
    # with open("pkmn/1.json", "r") as f:
        # data = json.load(f)
    #end

    #print(data)
    #print(data["name"])
    #print(data["id"])
#fi