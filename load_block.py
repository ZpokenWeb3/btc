# import requests
# import sys
# import json

# def get_full_block(height):
#     try:
#         url = f"https://api.blockchair.com/bitcoin/blocks?q=id({height})"
#         response = requests.get(url, timeout=10)
#         response.raise_for_status()
#         block_info = response.json()["data"][0]
#         block_hash = block_info["hash"]
#         block_url = f"https://api.blockchair.com/bitcoin/raw/block/{block_hash}"
#         block_data = requests.get(block_url, timeout=10).json()

#         return block_data

#     except requests.exceptions.RequestException as e:
#         print(f"Error: {e}")
#         return None

# def fetch_block_by_height(height):
#     """
#     Get block by height.
#     Use API Blockchain.info: https://blockchain.info/block-height/<height>?format=json
#     """
#     url = f"https://blockchain.info/block-height/{height}?format=json"
#     response = requests.get(url)
#     if response.status_code == 200:
#         data = response.json()
#         block = data.get("blocks", [None])[0]
#         return block
#     else:
#         print("Error loading block. Code:", response.status_code)
#         return None
    
# def fetch_block_by_hash(block_hash):
#     """
#     Get block by hash.
#     Use API Blockchain.info: https://blockchain.info/rawblock/<block_hash>
#     """
#     url = f"https://blockchain.info/rawblock/{block_hash}"
#     response = requests.get(url)
#     if response.status_code == 200:
#         block = response.json()
#         return block
#     else:
#         print("Error loading block. Code:", response.status_code)
#         return None

# def main():
#     if len(sys.argv) < 3:
#         print("Use python fetch_block.py <mode> <value>")
#         print("  mode: 'height' to get block by height or 'hash' to get block by hash")
#         sys.exit(1)

#     mode = sys.argv[1].lower()
#     value = sys.argv[2]

#     if mode == "height":
#         # block = get_full_block(value)
#         block = fetch_block_by_height(value)
#     elif mode == "hash":
#         block = fetch_block_by_hash(value)
#     else:
#         print("First argument should be 'height' or 'hash'")
#         sys.exit(1)

#     if block:
#         block_json = json.dumps(block, indent=4, ensure_ascii=False)
#         with open(f"blocks/block_{value}.json", "w", encoding="utf-8") as file:
#             file.write(block_json)  

#         print(f"Block is saved to blocks/block_{value}.json")

# if __name__ == "__main__":
#     main()

import requests

def download_block_as_hex(block_hash: str, filename: str):
    # Формируем URL для скачивания блока в raw-формате
    url = f"https://api.blockchair.com/bitcoin/raw/block/{block_hash}"

    try:
        # Отправляем GET запрос к API Blockchair
        response = requests.get(url)

        # Проверяем, что запрос был успешным
        if response.status_code == 200:
            # Получаем блок в сыром (hex) формате
            block_data = response.json().get('data', {}).get(block_hash, {}).get('raw', None)
            
            if block_data:
                # Сохраняем raw hex данные в файл
                with open(filename, 'w') as file:
                    file.write(block_data)
                print(f"Block data has been downloaded and saved as hex to '{filename}'.")
            else:
                print("Failed to extract raw block data.")
        else:
            print(f"Failed to download block data. Status code: {response.status_code}")
    except requests.exceptions.RequestException as e:
        print(f"An error occurred while downloading the block: {e}")

if __name__ == "__main__":
    # Пример хеша блока (замените на нужный хеш)
    block_hash = "000000000000a609f0b57fc9f6b30797a02d0729f4a7835dd0c7fbb87387a836"

    # Имя файла для сохранения (hex файл)
    filename = "block_data.hex"

    # Скачать блок в hex формате и сохранить в файл
    download_block_as_hex(block_hash, filename)
