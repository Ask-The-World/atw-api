from flask import Flask, json, request, jsonify

companies = [{"id": 1, "name": "Company One"}, {"id": 2, "name": "Company Two"}]

api = Flask(__name__)

@api.route('/', methods=['GET'])
def get_companies():
    if 'id' in request.args:
        try:
            id = int(request.args['id'])
            return jsonify(id**id)
        except Error as identifier:
            return jsonify(identifier)
    else:
        return json.dumps(companies)

if __name__ == '__main__':
    api.run()