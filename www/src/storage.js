export default{
    getBestScore : () => parseInt(localStorage.getBestScore) || 0,
    setBestScore : (bestScore) => localStorage.setItem('bestScore', bestScore)
}