#![no_std]
#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_doc_comments)]
#![allow(non_upper_case_globals)]
//! BOREALIS GENERATED FILE
extern crate alloc;
use AArch32_CommonFaultStatus::*;
use IFSR_write::*;
use TTBCR_read::*;
use Mk_IFSR_Type::*;
use u_get_TTBCR_Type_EAE::*;
use IFAR_write::*;
use CurrentSecurityState::*;
use IFAR_S_write::*;
use common::*;
pub fn AArch32_ReportPrefetchAbort<T: Tracer>(
    state: &mut State,
    tracer: &T,
    route_to_monitor: bool,
    fault: ProductType1d757adad216cdef,
    vaddress: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_9576: bool,
        gs_9577: bool,
        long_format: bool,
        gs_9574: bool,
        fsr: u32,
        route_to_monitor: bool,
        fault: ProductType1d757adad216cdef,
        vaddress: u32,
    }
    let fn_state = FunctionState {
        route_to_monitor,
        fault,
        vaddress,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #0u : u8
        let s_0_0: bool = false;
        // D s_0_1: write-var long_format <= s_0_0
        fn_state.long_format = s_0_0;
        // D s_0_2: read-var route_to_monitor:u8
        let s_0_2: bool = fn_state.route_to_monitor;
        // N s_0_3: branch s_0_2 b14 b1
        if s_0_2 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#9574 <= s_1_0
        fn_state.gs_9574 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#9574:u8
        let s_2_0: bool = fn_state.gs_9574;
        // N s_2_1: branch s_2_0 b7 b3
        if s_2_0 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #() : ()
        let s_3_0: () = ();
        // S s_3_1: call TTBCR_read(s_3_0)
        let s_3_1: ProductType700c18a878c5601b = TTBCR_read(state, tracer, s_3_0);
        // S s_3_2: call _get_TTBCR_Type_EAE(s_3_1)
        let s_3_2: bool = u_get_TTBCR_Type_EAE(state, tracer, s_3_1);
        // S s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 1u16);
        // C s_3_4: const #1u : u8
        let s_3_4: bool = true;
        // C s_3_5: cast zx s_3_4 -> bv
        let s_3_5: Bits = Bits::new(s_3_4 as u128, 1u16);
        // S s_3_6: cmp-eq s_3_3 s_3_5
        let s_3_6: bool = ((s_3_3) == (s_3_5));
        // D s_3_7: write-var long_format <= s_3_6
        fn_state.long_format = s_3_6;
        // N s_3_8: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var fault:struct
        let s_4_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_4_1: read-var long_format:u8
        let s_4_1: bool = fn_state.long_format;
        // D s_4_2: call AArch32_CommonFaultStatus(s_4_0, s_4_1)
        let s_4_2: u32 = AArch32_CommonFaultStatus(state, tracer, s_4_0, s_4_1);
        // D s_4_3: write-var fsr <= s_4_2
        fn_state.fsr = s_4_2;
        // D s_4_4: read-var route_to_monitor:u8
        let s_4_4: bool = fn_state.route_to_monitor;
        // N s_4_5: branch s_4_4 b6 b5
        if s_4_4 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var fsr:u32
        let s_5_0: u32 = fn_state.fsr;
        // D s_5_1: call Mk_IFSR_Type(s_5_0)
        let s_5_1: ProductType700c18a878c5601b = Mk_IFSR_Type(state, tracer, s_5_0);
        // D s_5_2: call IFSR_write(s_5_1)
        let s_5_2: () = IFSR_write(state, tracer, s_5_1);
        // D s_5_3: read-var vaddress:u32
        let s_5_3: u32 = fn_state.vaddress;
        // D s_5_4: call IFAR_write(s_5_3)
        let s_5_4: () = IFAR_write(state, tracer, s_5_3);
        // N s_5_5: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var fsr:u32
        let s_6_0: u32 = fn_state.fsr;
        // D s_6_1: call Mk_IFSR_Type(s_6_0)
        let s_6_1: ProductType700c18a878c5601b = Mk_IFSR_Type(state, tracer, s_6_0);
        // C s_6_2: const #11016u : u32
        let s_6_2: u32 = 11016;
        // N s_6_3: write-reg s_6_2 <= s_6_1
        let s_6_3: () = {
            state.write_register::<ProductType700c18a878c5601b>(s_6_2 as isize, s_6_1);
            tracer.write_register(s_6_2 as isize, s_6_1);
        };
        // D s_6_4: read-var vaddress:u32
        let s_6_4: u32 = fn_state.vaddress;
        // D s_6_5: call IFAR_S_write(s_6_4)
        let s_6_5: () = IFAR_S_write(state, tracer, s_6_4);
        // N s_6_6: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #15368u : u32
        let s_7_0: u32 = 15368;
        // D s_7_1: read-reg s_7_0:struct
        let s_7_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_7_0 as isize);
            tracer.read_register(s_7_0 as isize, value);
            value
        };
        // D s_7_2: call _get_TTBCR_Type_EAE(s_7_1)
        let s_7_2: bool = u_get_TTBCR_Type_EAE(state, tracer, s_7_1);
        // D s_7_3: cast zx s_7_2 -> bv
        let s_7_3: Bits = Bits::new(s_7_2 as u128, 1u16);
        // C s_7_4: const #1u : u8
        let s_7_4: bool = true;
        // C s_7_5: cast zx s_7_4 -> bv
        let s_7_5: Bits = Bits::new(s_7_4 as u128, 1u16);
        // D s_7_6: cmp-eq s_7_3 s_7_5
        let s_7_6: bool = ((s_7_3) == (s_7_5));
        // N s_7_7: branch s_7_6 b13 b8
        if s_7_6 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #16975u : u32
        let s_8_0: u32 = 16975;
        // D s_8_1: read-reg s_8_0:u8
        let s_8_1: u8 = {
            let value = state.read_register::<u8>(s_8_0 as isize);
            tracer.read_register(s_8_0 as isize, value);
            value
        };
        // D s_8_2: cast zx s_8_1 -> bv
        let s_8_2: Bits = Bits::new(s_8_1 as u128, 2u16);
        // C s_8_3: const #432u : u32
        let s_8_3: u32 = 432;
        // D s_8_4: read-reg s_8_3:u8
        let s_8_4: u8 = {
            let value = state.read_register::<u8>(s_8_3 as isize);
            tracer.read_register(s_8_3 as isize, value);
            value
        };
        // D s_8_5: cast zx s_8_4 -> bv
        let s_8_5: Bits = Bits::new(s_8_4 as u128, 2u16);
        // D s_8_6: cmp-eq s_8_2 s_8_5
        let s_8_6: bool = ((s_8_2) == (s_8_5));
        // D s_8_7: write-var gs#9576 <= s_8_6
        fn_state.gs_9576 = s_8_6;
        // N s_8_8: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#9576:u8
        let s_9_0: bool = fn_state.gs_9576;
        // N s_9_1: branch s_9_0 b12 b10
        if s_9_0 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #() : ()
        let s_10_0: () = ();
        // S s_10_1: call TTBCR_read(s_10_0)
        let s_10_1: ProductType700c18a878c5601b = TTBCR_read(state, tracer, s_10_0);
        // S s_10_2: call _get_TTBCR_Type_EAE(s_10_1)
        let s_10_2: bool = u_get_TTBCR_Type_EAE(state, tracer, s_10_1);
        // S s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 1u16);
        // C s_10_4: const #1u : u8
        let s_10_4: bool = true;
        // C s_10_5: cast zx s_10_4 -> bv
        let s_10_5: Bits = Bits::new(s_10_4 as u128, 1u16);
        // S s_10_6: cmp-eq s_10_3 s_10_5
        let s_10_6: bool = ((s_10_3) == (s_10_5));
        // D s_10_7: write-var gs#9577 <= s_10_6
        fn_state.gs_9577 = s_10_6;
        // N s_10_8: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#9577:u8
        let s_11_0: bool = fn_state.gs_9577;
        // D s_11_1: write-var long_format <= s_11_0
        fn_state.long_format = s_11_0;
        // N s_11_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #1u : u8
        let s_12_0: bool = true;
        // D s_12_1: write-var gs#9577 <= s_12_0
        fn_state.gs_9577 = s_12_0;
        // N s_12_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #1u : u8
        let s_13_0: bool = true;
        // D s_13_1: write-var gs#9576 <= s_13_0
        fn_state.gs_9576 = s_13_0;
        // N s_13_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #() : ()
        let s_14_0: () = ();
        // S s_14_1: call CurrentSecurityState(s_14_0)
        let s_14_1: u32 = CurrentSecurityState(state, tracer, s_14_0);
        // C s_14_2: const #3u : u32
        let s_14_2: u32 = 3;
        // S s_14_3: cmp-eq s_14_1 s_14_2
        let s_14_3: bool = ((s_14_1) == (s_14_2));
        // D s_14_4: write-var gs#9574 <= s_14_3
        fn_state.gs_9574 = s_14_3;
        // N s_14_5: jump b2
        return block_2(state, tracer, fn_state);
    }
}
