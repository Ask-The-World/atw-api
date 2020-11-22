# AskTheWorld - Table of Contents

- [AskTheWorld - Table of Contents](#asktheworld---table-of-contents)
- [AskTheWorld - General Information](#asktheworld---general-information)
  - [Idea](#idea)
  - [Security](#security)
  - [Method](#method)
  - [Use](#use)
    - [Question Mode](#question-mode)
    - [Answer mode](#answer-mode)
  - [Future](#future)
- [AskTheWorld - Docs](#asktheworld---docs)
  - [Configuration](#configuration)
    - [min_time](#min_time)
    - [max_time](#max_time)
    - [default_time](#default_time)
    - [max_question_lenght](#max_question_lenght)
    - [default_delete_time](#default_delete_time)
  - [API Functions](#api-functions)
    - [Submit Question - 1](#submit-question---1)
    - [Get Question - 2](#get-question---2)
    - [Answer Question - 3](#answer-question---3)
    - [Get Answer - 4](#get-answer---4)

# AskTheWorld - General Information

## Idea

This api allows the user to make quick anonymous surveys with random persons from all over the world answering their questions.

This tool is related to the movie "Unknown User: Dark Web", where a command tool named FlashVote lets people decide about the life of the main character Matias.

## Security

The server does not store any private information from users like their IPs so the surveys will be fully anonymous. Because of this AskTheWorld is not responsible for any contents of the questions nor the answers.

## Method

This api needs to be able to serve many users at the same time although running on a slow system (Raspberry Pi 4). For this reason the API will be written in Rust and utilizes the Actix web server, which currently is the fastest in the world.

If there are as many votes for yes as there are for no the api will randomly decide what to answer.

## Use

### Question Mode

In question mode the user is able to enter a question which cannot be longer than a specific number of characters. The only available answers to these questions will be yes and no. The user can also specify a time limit after which the survey ends. This limit is being restricted by a limiting range specified in the [configuration](#configuration). After submiting his question the user can see the number of results and the time left until the survey ends. At the end he will only see the amount of received answers and the final result.

### Answer mode

When the user enters answer mode he will be presented a randomly selected question and can decide between yes or no. He will also be able to see the remaining time so he does not run out of time before deciding.

## Future

Right now the main priority is developing the backend server as well as the api. This is probably going to be achieved by using Rust script which is running on a Raspberry Pi 4 server. The stability of this system is not great so the server will probably be changed as soon as there are more users. To further make it possible for the user to use this api, there will also be a command line tool written in Bash which can be used to send GET request to the api. After all of these programs work reliably there also might be a website where all users can use this tool, regardless of their Operating System and command line experience.

# AskTheWorld - Docs

## Configuration

---

### min_time

Minimum required duration in seconds to be specified for a question after which no answer can be submitted anymore. If duration is below [min_time](#min_time) the [default_time](#default_time) will be used.

### max_time

Maximum required duration in seconds to be specified for a question after which no answer can be submitted anymore. If duration is above [max_time](#max_time) the [default_time](#default_time) will be used.

### default_time

This is the default duration in seconds while which answers may be submitted for a question. If no duration is specified when the question is being submitted this time will be used.

### max_question_lenght

This is the maximum lenght the question may have specified as the number of maximum characters. If the question is longer than this it will throw the 201 error.

### default_delete_time

This is the default time after which all questions will be deleted from the database. Please note, that it will be added to the time left to submit questions.

## API Functions

---

### Submit Question - 1

- Args:
    - int: 1 for submitting mode
    - String: question
    - int: time until survey ends
- Output:
    - ObjectId of question in database
    - String: status
        - 200 - everything worked fine
        - 201 - question was longer than [max_question_length](#max_question_lenght)
        - 202 - no question submitted
        - 203 - no time submitted, used [default_time](#default_time) instead
        - 204 - submitted time was outside of possible range specified by [min_time](#min_time) and [max_time](#max_time) so [default_time](#default_time) was used
        - 205 - could not find the question in the database, it might have already been deleted or the ObjectId has been wrong

### Get Question - 2

- Args: 
    - int: 2 for getting question mode
- Output:
    - String: status
        - 200 - everything worked fine
    - ObjectId of question in database so an answer can be submitted
    - String: question
    - int: time left until no answers can be submitted in seconds

### Answer Question - 3

- Args:
    - int: 3 for answering mode
    - Bool: submitted answer, True for yes and False for no
    - ObjectId of question to match the answer to the question
- Output:
    - String: status
        - 200 - everything worked fine
        - 205 - could not find the question in the database, it might have already been deleted or the ObjectId has been wrong
        - 206 - time of question ran out before answer has been submitted
  
### Get Answer - 4

- Args:
    - int: 4 for getting answer mode
    - ObjectId of the question
- Output:
    - String: status
        - 200 - everything worked fine
        - 205 - could not find the question in the database, it might have already been deleted or the ObjectId has been wrong
    - String: question
    - int: time left until submitting is no longer possible in seconds
    - Bool: answer for the question