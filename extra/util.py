import requests

def saveInput(dayno):
    envpairs = {}
    with open("../.env") as f:
        envpairs = { k:v for k,v in [l.strip().split("=") for l in f.readlines()]}
    aockey = envpairs.get("AOC_SESSION_ID")
    endpoint = f"https://adventofcode.com/2022/day/{dayno}/input"
    session_cookie = {"session": aockey}
    inputtext = requests.get(endpoint, cookies=session_cookie).text
    with open(f"../inputs/{dayno:02}.txt", "w") as f:
        f.write(inputtext)



