# AskTheWorld

## Idea

This api allows the user to make quick anonymous surveys with random persons from all over the world answering their questions.

This tool is related to the movie "Unknown User: Dark Web", where a command tool named FlashVote lets people decide about the life of the main character Matias.

## Security

The server does not store any private information from users like their IPs so the surveys will be fully anonymous. Because of this AskTheWorld is not responsible for any contents of the questions nor the answers.

## Use

### Question Mode

In question mode the user is able to enter a question which cannot be longer than 255 characters. The only available answers to these questions will be yes and no. The user can also specify a time limit after which the survey ends. This limit can range from 10 seconds to 5 minutes. After submiting his question the user can see the number of results and the time left until the survey ends. At the end he will only see the amount of received answers and the final result.

### Answer mode

When the user enters answer mode he will be presented a randomly selected question and can decide between yes or no. He will also be able to see the remaining time so he does not run out of time before deciding.

## Future

Right now the main priority is developing the backend server as well as the api. This is probably going to be achieved by using a Python script which is running on a Raspberry Pi 4 server. The stability of this system is not great so the server will probably be changed as soon as there are more users. To further make it possible for the user to use this api, there will also be a command line tool written in Bash which can be used to send POST and GET request to the api. After all of these programs work reliably there also might be a website where all users can use this tool, regardless of their Operating System and command line experience.