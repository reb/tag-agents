from utils import Action, distance


def decide_action(is_it, position, tagging_agent_position, other_agent_positions=[]):
    # Try to tag other agents
    if is_it:
        # TODO: Write logic for when the agent is it
        return Action.Random.value

    # Meander around if there's no risk of being tagged
    if distance(position, tagging_agent_position) > 8:
        return Action.Random.value

    # Move away from the tagging agent, prioritizing the direction it's closest in
    if abs(position[0] - tagging_agent_position[0]) < abs(position[1] - tagging_agent_position[1]):
        if position[0] < tagging_agent_position[0]:
            return Action.Left.value
        else:
            return Action.Right.value
    else:
        if position[1] < tagging_agent_position[1]:
            return Action.Up.value
        else:
            return Action.Down.value
