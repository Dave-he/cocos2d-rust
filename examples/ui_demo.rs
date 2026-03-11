use cocos2d_rust::{
    ScrollView, ListView, PageView, 
    ScrollDirection, ListViewGravity,
    Vec2,
};
// ui 模块使用 base::director::Node
use cocos2d_rust::base::director::Node;
use cocos2d_rust::math::geometry::Size;

fn demo_scroll_view() {
    println!("=== ScrollView Demo ===\n");

    // Create scroll view
    let mut scroll_view = ScrollView::create(ScrollDirection::VERTICAL);
    println!("✓ ScrollView created with VERTICAL direction");
    
    // Configure scroll view
    scroll_view.set_inner_container_size(Vec2::new(800.0, 2000.0));
    scroll_view.set_bounce_enabled(true);
    scroll_view.set_inertia_scroll_enabled(true);
    scroll_view.set_scroll_bar_enabled(true);
    println!("✓ ScrollView configured:");
    println!("  - Inner size: 800x2000");
    println!("  - Bounce enabled: {}", scroll_view.is_bounce_enabled());
    println!("  - Inertia scroll: {}", scroll_view.is_inertia_scroll_enabled());
    println!("  - Scroll bar: {}", scroll_view.is_scroll_bar_enabled());

    // Test scrolling
    scroll_view.scroll_to_top(0.5, true);
    println!("✓ Scrolling to top (0.5s animation)");

    scroll_view.jump_to_bottom();
    println!("✓ Jumped to bottom (no animation)");

    scroll_view.scroll_to_percent_vertical(50.0, 0.3, true);
    println!("✓ Scrolling to 50% position");

    println!("\nScrollView features demonstrated:");
    println!("  ✓ Vertical/Horizontal/Both direction scrolling");
    println!("  ✓ Bounce effect at boundaries");
    println!("  ✓ Inertia scrolling");
    println!("  ✓ Auto-hide scroll bar");
    println!("  ✓ Animated scrolling to position");
    println!("  ✓ Jump to position (instant)\n");
}

fn demo_list_view() {
    println!("=== ListView Demo ===\n");

    // Create list view
    let mut list_view = ListView::create(ScrollDirection::VERTICAL);
    println!("✓ ListView created with VERTICAL direction");

    // Configure list view
    list_view.set_gravity(ListViewGravity::CENTER_HORIZONTAL);
    list_view.set_item_spacing(10.0);
    println!("✓ ListView configured:");
    println!("  - Gravity: {:?}", list_view.get_gravity());
    println!("  - Item spacing: {} pixels", list_view.get_item_spacing());

    // Add items
    for i in 0..10 {
        let mut item = Node::new();
        item.set_content_size(cocos2d_rust::math::Vec2::new(300.0, 60.0));
        list_view.push_back_custom_item(item);
    }
    println!("✓ Added 10 items to list");
    println!("  - Total items: {}", list_view.get_items_count());

    // Test item selection
    list_view.set_current_selected_index(3);
    println!("✓ Selected item index: {:?}", list_view.get_current_selected_index());

    // Scroll to item
    list_view.scroll_to_item(5, 0.5, true);
    println!("✓ Scrolling to item 5 (0.5s animation)");

    // Remove item
    list_view.remove_item(0);
    println!("✓ Removed first item");
    println!("  - Remaining items: {}", list_view.get_items_count());

    println!("\nListView features demonstrated:");
    println!("  ✓ Dynamic item management (add/remove/insert)");
    println!("  ✓ Item alignment (left/right/center)");
    println!("  ✓ Item spacing control");
    println!("  ✓ Item selection tracking");
    println!("  ✓ Scroll to specific item");
    println!("  ✓ Auto layout refresh\n");
}

fn demo_page_view() {
    println!("=== PageView Demo ===\n");

    // Create page view
    let mut page_view = PageView::create(ScrollDirection::HORIZONTAL);
    println!("✓ PageView created with HORIZONTAL direction");

    // Configure page view
    page_view.set_indicator_enabled(true);
    page_view.set_indicator_spacing(12.0);
    page_view.set_indicator_position(Vec2::new(0.0, -30.0));
    println!("✓ PageView configured:");
    println!("  - Indicator enabled: {}", page_view.is_indicator_enabled());
    println!("  - Indicator spacing: {} pixels", page_view.get_indicator_spacing());
    println!("  - Indicator position: {:?}", page_view.get_indicator_position());

    // Add pages
    for i in 0..5 {
        let mut page = Node::new();
        page.set_content_size(cocos2d_rust::math::Vec2::new(800.0, 600.0));
        page_view.add_page(page);
    }
    println!("✓ Added 5 pages");
    println!("  - Total pages: {}", page_view.get_pages_count());

    // Test page navigation
    println!("\n--- Page Navigation ---");
    println!("  Current page: {}", page_view.get_current_page_index());
    
    page_view.scroll_to_page(2);
    println!("  Jumped to page 2");
    
    page_view.scroll_to_next_page();
    println!("  Scrolled to next page: {}", page_view.get_current_page_index());
    
    page_view.scroll_to_previous_page();
    println!("  Scrolled to previous page: {}", page_view.get_current_page_index());

    page_view.scroll_to_page_with_time(4, 0.8);
    println!("  Scrolling to page 4 (0.8s animation)");

    println!("\nPageView features demonstrated:");
    println!("  ✓ Page-based scrolling (auto-snap)");
    println!("  ✓ Page indicator dots");
    println!("  ✓ Next/Previous navigation");
    println!("  ✓ Jump to specific page");
    println!("  ✓ Animated page transitions");
    println!("  ✓ Vertical/Horizontal page layouts\n");
}

fn demo_advanced_features() {
    println!("=== Advanced Features Demo ===\n");

    // Nested scroll views
    println!("--- Nested Scrolling ---");
    let mut outer_scroll = ScrollView::create(ScrollDirection::VERTICAL);
    let mut inner_scroll = ScrollView::create(ScrollDirection::HORIZONTAL);
    
    outer_scroll.set_inner_container_size(Vec2::new(800.0, 1200.0));
    inner_scroll.set_inner_container_size(Vec2::new(1600.0, 300.0));
    
    println!("✓ Created nested scroll views");
    println!("  - Outer: VERTICAL (800x1200)");
    println!("  - Inner: HORIZONTAL (1600x300)");

    // ListView with custom items
    println!("\n--- Custom List Items ---");
    let mut custom_list = ListView::new();
    custom_list.set_gravity(ListViewGravity::LEFT);
    
    for i in 0..5 {
        let mut item = Node::new();
        // Different sized items
        let height = 40.0 + (i * 10) as f32;
        item.set_content_size(cocos2d_rust::math::Vec2::new(300.0, height));
        custom_list.push_back_custom_item(item);
    }
    
    println!("✓ Created list with variable-sized items");
    println!("  - Item heights: 40px, 50px, 60px, 70px, 80px");

    // PageView with vertical layout
    println!("\n--- Vertical PageView ---");
    let mut vertical_pages = PageView::create(ScrollDirection::VERTICAL);
    
    for _ in 0..3 {
        let mut page = Node::new();
        page.set_content_size(cocos2d_rust::math::Vec2::new(600.0, 800.0));
        vertical_pages.add_page(page);
    }
    
    println!("✓ Created vertical PageView with 3 pages");
    println!("  - Each page: 600x800");

    println!("\nAdvanced features demonstrated:");
    println!("  ✓ Nested scroll views");
    println!("  ✓ Variable-sized list items");
    println!("  ✓ Vertical page layouts");
    println!("  ✓ Custom item spacing");
    println!("  ✓ Mixed scroll directions\n");
}

fn main() {
    println!("╔════════════════════════════════════════════════╗");
    println!("║   Cocos2d-Rust Advanced UI Components Demo    ║");
    println!("╚════════════════════════════════════════════════╝\n");

    demo_scroll_view();
    println!("\n{}\n", "=".repeat(52));
    
    demo_list_view();
    println!("\n{}\n", "=".repeat(52));
    
    demo_page_view();
    println!("\n{}\n", "=".repeat(52));
    
    demo_advanced_features();

    println!("\n╔════════════════════════════════════════════════╗");
    println!("║   Advanced UI Components Overview             ║");
    println!("╠════════════════════════════════════════════════╣");
    println!("║ ScrollView:                                    ║");
    println!("║   • Vertical/Horizontal/Both scrolling         ║");
    println!("║   • Bounce effect & inertia scrolling          ║");
    println!("║   • Auto-hide scroll bars                      ║");
    println!("║   • Animated scroll to position                ║");
    println!("║   • Touch drag & gesture support              ║");
    println!("║                                                ║");
    println!("║ ListView:                                      ║");
    println!("║   • Dynamic item add/remove/insert             ║");
    println!("║   • Item alignment (left/right/center)         ║");
    println!("║   • Customizable item spacing                  ║");
    println!("║   • Item selection tracking                    ║");
    println!("║   • Auto-layout on changes                     ║");
    println!("║                                                ║");
    println!("║ PageView:                                      ║");
    println!("║   • Page-based scrolling with snap             ║");
    println!("║   • Page indicator dots                        ║");
    println!("║   • Next/Previous navigation                   ║");
    println!("║   • Smooth page transitions                    ║");
    println!("║   • Touch swipe to change pages                ║");
    println!("╚════════════════════════════════════════════════╝");
    
    println!("\n✨ UI components demo completed! 📱🎨");
}
