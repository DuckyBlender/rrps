# import the necessary packages
from flask import Flask
import subprocess

# Initialize the Flask application
app = Flask(__name__)

# Define the stream URL
stream_url = 'https://stream.open.fm/64'

# Define a variable to store the media player process
player_process = None

# Define the endpoint to start the audio
@app.route('/start')
def start_audio():
    # Start the player process
    global player_process
    # Check if the player process is already running
    if player_process is not None:
        # Return an error message
        return 'Audio is already playing!'
    player_process = subprocess.Popen(
        ['mpv', stream_url], stdin=subprocess.PIPE, stdout=subprocess.PIPE)

    # Return a success message
    return 'Audio started successfully!'

# Define the endpoint to stop the audio


@app.route('/stop')
def stop_audio():
    # Stop the player process
    global player_process
    if player_process is not None:
        player_process.stdin.write(b'q')
        player_process.terminate()
        player_process = None

        # Return a success message
        return 'Audio stopped successfully!'
    else:
        # Return an error message
        return 'No audio is playing!'

# Define the endpoint to change the volume
@app.route('/volume')
def get_volume():
    # Get the volume using amixer
    volume = subprocess.check_output(['amixer', '-D', 'pulse', 'sget', 'Master'])
    # Extract the volume level from the output
    volume_level = int(volume.split(b'[')[1].split(b'%')[0])
    # Return the volume level
    return str(volume_level)

@app.route('/volume/<volume_level>')
def change_volume(volume_level):
    # Check if the volume level is an integer
    if not volume_level.isdigit():
        # Return an error message
        return 'Volume level must be an integer!'
    # Check if the volume level is between 0 and 100
    if int(volume_level) < 0 or int(volume_level) > 100:
        # Return an error message
        return 'Volume level must be between 0 and 100!'
    

    # Change the volume using amixer
    subprocess.Popen(['amixer', 'sset', 'Master', volume_level + '%'], shell=True, stdout=subprocess.PIPE).wait()

    # Return a success message
    return 'Volume changed successfully!'


# Run the Flask application
if __name__ == '__main__':
    app.run(host='0.0.0.0', port=5000)
