export interface RoomInfo {
    roomID: string,
    roomName: string,
    users: UserInfo[],
    avatar?: string,
    unreadCount?: number,
    index?: number,
    lastMessage?: MessageInfo,
    typingUsers?: string[],
}

export interface UserInfo {
    _id: string,
    username: string,
    avatar?: string,
    status?: UserStatus,
}

export interface UserStatus {
    state: string, // state can be 'online' or 'offline'
    lastChanged?: string, // lastChanged is the date when state was last modified.
}

export interface MessageInfo {
    _id: string,
    indexId: number, // can be used if you need to change a message ID that is already displayed in a room, this preventing an animation glitch. For example, when you don't know in advance the message ID your backend will create.
    content: string,
    senderId: string, // Each message object has a senderId field which holds the id of the corresponding agent. If senderId matches the currentUserId prop, specific UI and actions will be implemented.
    username: string,
    avatar: string,
    date: string,
    timestamp: string,
    system: boolean, //  is used to show messages with a specific centered display
    saved: boolean, // one checkmark
    distributed: boolean, // two checkmarks
    seen: boolean,  // two blue checkmarks
    deleted: boolean, // grey background with deleted message text
    failure: boolean, // red clickable failure icon
    disableActions: boolean,
    disableReactions: boolean,
}

export function newMessageInfo(_id: string,
    indexId: number, // can be used if you need to change a message ID that is already displayed in a room, this preventing an animation glitch. For example, when you don't know in advance the message ID your backend will create.
    content: string,
    senderId: string, // Each message object has a senderId field which holds the id of the corresponding agent. If senderId matches the currentUserId prop, specific UI and actions will be implemented.
    username: string,
    avatar: string,
    date: string,
    timestamp: string,
    saved: boolean, // one checkmark
    distributed: boolean, // two checkmarks
    seen: boolean,  // two blue checkmarks
    deleted: boolean, // grey background with deleted message text
    failure: boolean, // red clickable failure icon
    system: boolean, //  is used to show messages with a specific centered display
    disableActions: boolean,
    disableReactions: boolean,): MessageInfo {
    return {
        _id,
        indexId,
        content,
        senderId,
        username,
        avatar,
        date,
        timestamp,
        saved,
        distributed,
        seen,
        deleted,
        failure,
        system,
        disableActions,
        disableReactions,
    }
}