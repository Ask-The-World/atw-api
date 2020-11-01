from flask import Flask, json, request, jsonify
import pymongo
from pymongo import ReturnDocument
import bson
import random
import time as t
from threading import Timer

threads = []

myclient = pymongo.MongoClient('mongodb://localhost:27017/')

mydb = myclient['test']

mycol = mydb['collection']

result = mycol.find_one()

api = Flask(__name__)

def kill(id):
    mycol.delete_one({'_id': id})

@api.route('/', methods=['GET'])
def Api():
    # Make a new question
    if 'question' in request.args:
        try:
            question = str(request.args['question'])
            time = int(request.args['time'])
            stamp = int(t.time())
            mydict = {'yes': 0, 'no': 0, 'time': time, 'question': question, 'stamp': stamp}
            result = mycol.insert_one(mydict)
            id = result.inserted_id
            thread = Timer(time + 50, kill, args = [id])
            threads.append(thread)
            threads[-1].start()
            return jsonify(str(id))
        except:
            return jsonify('406')

    # Answer an existing question
    elif 'answer' in request.args:
        try:
            answer = bool(request.args['answer'])
            id = str(request.args['id'])
            _id = bson.objectid.ObjectId(id)
            if answer:
                result = mycol.find_one_and_update({'_id': _id}, {'$inc': {'yes': 1}}, return_document=ReturnDocument.AFTER)
                return jsonify("Success")
            else:
                result = mycol.find_one_and_update({'_id': _id}, {'$inc': {'no': 1}}, return_document=ReturnDocument.AFTER)
                return jsonify("Success")
        except Exception as e:
            print(e)
            return jsonify('408')
    
    # Look up the results of a question
    elif 'id' in request.args:
        try:
            id = str(request.args['id'])
            _id = bson.objectid.ObjectId(id)
            result = mycol.find_one({'_id': _id})
            seconds_left = int(result['stamp']) + int(result['time']) - int(t.time())
            number_of_votes = int(result['yes']) + int(result['no'])
            if int(result['yes']) > int(result['no']):
                answer = "yes"
            elif int(result['yes']) < int(result['no']):
                answer = "no"
            elif bool(random.getrandbits(1)):
                answer = "yes"
            else:
                answer = "no"
            return jsonify(result['question'], seconds_left, number_of_votes, answer)
        except Exception as e:
            print(e)
            return jsonify('407')
    else:
        return jsonify('405')

if __name__ == '__main__':
    api.run(host="192.168.178.66", port=80)