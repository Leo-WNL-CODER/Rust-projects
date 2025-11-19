import { useEffect, useState } from "react";
import axios from "axios";

function App() {
  const [todos, setTodos] = useState([]);
  const [description, setDescription] = useState("");
  const [refresh, setRefresh] = useState(false);
  const [version, setVersion] = useState("v1");

  useEffect(() => {
    async function fetchTodos() {
      try {
        const res = await axios.get(`http://localhost:3000/${version}`);
        setTodos(res.data);
      } catch (err) {
        console.error("Error fetching todos:", err);
      } finally {
        setRefresh(false);
      }
    }
    fetchTodos();
  }, [refresh, version]);

  const handleAddTodo = async () => {
    if (!description.trim()) return;
    try {
      await axios.post(`http://localhost:3000/${version}/addTodo`, {
        description,
      });
      setDescription("");
      setRefresh(true);
    } catch (err) {
      console.error("Error adding todo:", err);
    }
  };

  const handleDelete = async (id:number) => {
    try {
      await axios.delete(`http://localhost:3000/${version}/deleteTodo/${id}`);
      setRefresh(true);
    } catch (err) {
      console.error("Error deleting todo:", err);
    }
  };

  const handleMarkDone = async (id:number) => {
    try {
      await axios.put(`http://localhost:3000/${version}/markDone/${id}`);
      setRefresh(true);
    } catch (err) {
      console.error("Error marking todo done:", err);
    }
  };

  return (
    <div className="min-h-screen bg-gradient-to-br from-blue-50 via-indigo-50 to-purple-50 py-12 px-4">
      <div className="max-w-3xl mx-auto">
        {/* Header */}
        <div className="text-center mb-12">
          <h1 className="text-5xl font-bold text-gray-800 mb-2">
            My Todo List
          </h1>
          <p className="text-gray-600">Organize your tasks efficiently</p>
          
          {/* Version Switcher */}
          <div className="mt-6 inline-flex bg-white rounded-xl shadow-md p-1">
            <button
              onClick={() => setVersion("v1")}
              className={`px-6 py-2 rounded-lg font-semibold transition-all duration-200 ${
                version === "v1"
                  ? "bg-indigo-500 text-white shadow-md"
                  : "text-gray-600 hover:text-gray-800"
              }`}
            >
              V1 (Memory)
            </button>
            <button
              onClick={() => setVersion("v2")}
              className={`px-6 py-2 rounded-lg font-semibold transition-all duration-200 ${
                version === "v2"
                  ? "bg-indigo-500 text-white shadow-md"
                  : "text-gray-600 hover:text-gray-800"
              }`}
            >
              V2 (Database)
            </button>
          </div>
        </div>

        {/* Add Todo Section */}
        <div className="bg-white rounded-2xl shadow-lg p-6 mb-8">
          <div className="flex gap-3">
            <input
              onChange={(e) => setDescription(e.target.value)}
              value={description}
              onKeyDown={(e) => e.key === "Enter" && handleAddTodo()}
              className="flex-1 px-4 py-3 border-2 border-gray-200 rounded-xl focus:border-indigo-400 focus:outline-none transition-colors"
              type="text"
              placeholder="What needs to be done?"
            />
            <button
              onClick={handleAddTodo}
              className="px-6 py-3 bg-indigo-500 text-white font-semibold rounded-xl hover:bg-indigo-600 transition-all duration-200 shadow-md hover:shadow-lg transform hover:-translate-y-0.5"
            >
              Add Task
            </button>
          </div>
        </div>

        {/* Todos List */}
        <div className="space-y-3">
          {todos.length === 0 ? (
            <div className="text-center py-12 bg-white rounded-2xl shadow-md">
              <div className="text-6xl mb-4">📝</div>
              <p className="text-gray-500 text-lg">No todos yet. Add one to get started!</p>
            </div>
          ) : (
            todos.map((t:{id:number,description:String,completed:boolean}, index) => (
              <div
                key={t.id}
                className="bg-white rounded-xl shadow-md hover:shadow-lg transition-all duration-200 p-5 group"
              >
                <div className="flex items-center justify-between gap-4">
                  {/* Todo Content */}
                  <div className="flex items-center flex-1 min-w-0">
                    <span className="flex-shrink-0 w-8 h-8 bg-indigo-100 text-indigo-600 rounded-full flex items-center justify-center font-semibold text-sm">
                      {index + 1}
                    </span>
                    <p
                      className={`ml-4 text-lg flex-1 ${
                        t.completed
                          ? "line-through text-gray-400"
                          : "text-gray-700"
                      }`}
                    >
                      {t.description}
                    </p>
                  </div>

                  {/* Action Buttons */}
                  <div className="flex gap-2 flex-shrink-0">
                    <button
                      disabled={t.completed}
                      onClick={() => handleMarkDone(t.id)}
                      className={`px-4 py-2 rounded-lg font-medium transition-all duration-200 ${
                        t.completed
                          ? "bg-green-100 text-green-700 cursor-not-allowed"
                          : "bg-blue-500 text-white hover:bg-blue-600 shadow-sm hover:shadow-md transform hover:-translate-y-0.5"
                      }`}
                    >
                      {t.completed ? "✓ Done" : "Complete"}
                    </button>
                    <button
                      onClick={() => handleDelete(t.id)}
                      className="px-4 py-2 bg-red-500 text-white rounded-lg font-medium hover:bg-red-600 transition-all duration-200 shadow-sm hover:shadow-md transform hover:-translate-y-0.5"
                    >
                      Delete
                    </button>
                  </div>
                </div>
              </div>
            ))
          )}
        </div>

        {/* Footer Stats */}
        {todos.length > 0 && (
          <div className="mt-8 text-center text-gray-600">
            <p>
              {todos.filter((t:{id:number,description:String,completed:boolean}) => t.completed).length} of {todos.length} tasks completed
            </p>
          </div>
        )}
      </div>
    </div>
  );
}

export default App;