def test_duplicate_args():
    endpoints = ["a","b","c","d"]
    args_endpoints = ["a","b","e"]
    # session = req_cache.CachedSession('pokeapi-cache') # pokeapi-cache.sqlite
    # add endpoints users wants to cache,


    # if endpoints are added multiple times it should be skipped to prevent duplication(idempotent)
    # endpoint = [a,b,c] arg=[a] -> skip a
    for arg in args_endpoints:
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
            print("appended " + arg)
        else:
            print("found duplicate " + arg + " skipping")
    #rof
    print("final: "+ "".join(endpoints))
#fed