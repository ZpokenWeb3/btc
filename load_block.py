import requests
import sys
import json

def get_full_block_height(height):
    try:
        url = f"https://blockchain.info/block-height/{height}?format=json"
        response = requests.get(url, timeout=10)
        response.raise_for_status()
        data = response.json()

        block = data["blocks"][0]

        complete_block = {
            "hash": block["hash"],
            "height": block["height"],
            "ver": block["ver"],
            "prev_block": block["prev_block"],
            "mrkl_root": block["mrkl_root"],
            "time": block["time"],
            "bits": block["bits"],
            "nonce": block["nonce"],
            "size": block["size"],
            "weight": block.get("weight"),
            "tx": block["tx"], 
        }
        return complete_block

    except requests.exceptions.RequestException as e:
        print(f"Error: {e}")
        return None

def get_full_block_hash(block_hash):
    try:
        url = f"https://blockchain.info/rawblock/{block_hash}"
        response = requests.get(url, timeout=10)
        response.raise_for_status()
        block = response.json()

        complete_block = {
            "hash": block["hash"],
            "height": block["height"],
            "ver": block["ver"],
            "prev_block": block["prev_block"],
            "mrkl_root": block["mrkl_root"],
            "time": block["time"],
            "bits": block["bits"],
            "nonce": block["nonce"],
            "size": block["size"],
            "weight": block.get("weight"),  
            "tx": block["tx"],  
        }

        return complete_block

    except requests.exceptions.RequestException as e:
        print(f"Error: {e}")
        return None
    
def fetch_block_by_height(height):
    """
    Get block by height.
    Use API Blockchain.info: https://blockchain.info/block-height/<height>?format=json
    """
    url = f"https://blockchain.info/block-height/{height}?format=json"
    response = requests.get(url)
    if response.status_code == 200:
        data = response.json()
        block = data.get("blocks", [None])[0]
        return block
    else:
        print("Error loading block. Code:", response.status_code)
        return None
    
def fetch_block_by_hash(block_hash):
    """
    Get block by hash.
    Use API Blockchain.info: https://blockchain.info/rawblock/<block_hash>
    """
    url = f"https://blockchain.info/rawblock/{block_hash}"
    response = requests.get(url)
    if response.status_code == 200:
        block = response.json()
        return block
    else:
        print("Error loading block. Code:", response.status_code)
        return None

def main():
    if len(sys.argv) < 3:
        print("Use python fetch_block.py <mode> <value>")
        print("  mode: 'height' to get block by height or 'hash' to get block by hash")
        sys.exit(1)

    mode = sys.argv[1].lower()
    value = sys.argv[2]

    if mode == "height":
        block = get_full_block_height(value)
    elif mode == "hash":
        block = get_full_block_hash(value)
    else:
        print("First argument should be 'height' or 'hash'")
        sys.exit(1)

    if block:
        block_json = json.dumps(block, indent=4, ensure_ascii=False)
        with open(f"blocks/block_{value}.json", "w", encoding="utf-8") as file:
            file.write(block_json)  

        print(f"Block is saved to blocks/block_{value}.json")

if __name__ == "__main__":
    main()

