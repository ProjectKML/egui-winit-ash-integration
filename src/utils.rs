use ash::vk;

pub fn insert_image_memory_barrier(
    device: &ash::Device,
    cmd_buff: &vk::CommandBuffer,
    image: &vk::Image,
    src_q_family_index: u32, dst_q_family_index: u32,
    src_access_mask: vk::AccessFlags, dst_access_mask: vk::AccessFlags,
    old_image_layout: vk::ImageLayout, new_image_layout: vk::ImageLayout,
    src_stage_mask: vk::PipelineStageFlags, dst_stage_mask: vk::PipelineStageFlags,
    subresource_range: vk::ImageSubresourceRange
) {

    let image_memory_barrier = vk::ImageMemoryBarrier::default()
        .src_queue_family_index(src_q_family_index)
        .dst_queue_family_index(dst_q_family_index)
        .src_access_mask(src_access_mask)
        .dst_access_mask(dst_access_mask)
        .old_layout(old_image_layout)
        .new_layout(new_image_layout)
        .image(*image)
        .subresource_range(subresource_range);
    unsafe {
        device.cmd_pipeline_barrier(*cmd_buff,
        src_stage_mask, dst_stage_mask,
        vk::DependencyFlags::BY_REGION,
        &[], &[], &[image_memory_barrier]);
    }
}