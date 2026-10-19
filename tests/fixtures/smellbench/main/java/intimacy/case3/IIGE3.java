package intimacy.case3;

import java.util.HashMap;
import java.util.Map;

class BookVariation {
    private String name;
    private String genre;
    private boolean isIssued;

    public BookVariation(String name, String genre) {
        this.name = name;
        this.genre = genre;
        this.isIssued = false;
    }

    public void issue() {
        this.isIssued = true;
    }

    public String getGenre() {
        return this.genre;
    }

    public String getName() {
        return this.name;
    }

    public boolean getIssueStatus() {
        return this.isIssued;
    }

    public void displayDetails() {
        System.out.println("Name: " + this.name);
        System.out.println("Genre: " + this.genre);
        System.out.println("Is issued: " + this.isIssued);
    }
}

class LibraryVariation {
    Map<Integer, BookVariation> books;

    public LibraryVariation() {
        this.books = new HashMap<>();
    }

    public void addBook(int id, BookVariation book) {
        this.books.put(id, book);
    }

    public void removeBook(int id) {
        this.books.remove(id);
    }

    public void issueBook(int id) {
        books.get(id).issue();
    }

    public void viewNonIssuedBooks() {
        for (Map.Entry<Integer, BookVariation> book : books.entrySet()) {
            if (book.getValue().getIssueStatus() == false) {
                book.getValue().displayDetails();
            }
        }
    }
}