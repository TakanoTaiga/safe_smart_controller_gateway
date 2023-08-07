from launch import LaunchDescription , actions
from launch_ros.actions import Node
from ament_index_python.packages import get_package_share_directory
import os.path


def generate_launch_description():
    MACRO_MDC_ASYNC = False
    MACRO_MDC_SYNC = True
    return LaunchDescription([
        # Physical Controller
        Node(
            package='safe_smart_controller_gateway',
            executable='safe_smart_controller_gateway',
            parameters=[{'port' : 64201} , 
                        {'debug' : False}],
            on_exit=actions.Shutdown(),
            remappings=[
                ('data_out' , 'scgw/out'),
            ]
        ),        
    ])