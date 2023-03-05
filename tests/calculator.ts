import * as anchor from "@project-serum/anchor"
import {Program} from "@project-serum/anchor"
import {Calculator} from "../target/types/calculator"

const {SystemProgram} = anchor.web3
import {expect} from "chai"

class BinOp {
	constructor(public name: string, public lhs: number, public rhs: number, public res: number) {
	}
}

describe("calculator", () => {
	// Configure the client to use the local cluster.
	anchor.setProvider(anchor.AnchorProvider.env())
	const program = anchor.workspace.Calculator as Program<Calculator>

	const programProvider = program.provider as anchor.AnchorProvider

	const calculatorPair = anchor.web3.Keypair.generate()

	const text = "Simple Calculator"

	it("create calculator instance", async () => {
		await program.methods.create(text).accounts({
			calculator: calculatorPair.publicKey,
			user: programProvider.wallet.publicKey,
			systemProgram: SystemProgram.programId,
		}).signers([calculatorPair]).rpc()

		const account = await program.account.calculator.fetch(calculatorPair.publicKey)
		expect(account.greeting).eq(text)
	})

	const binOperations = [
		new BinOp("division", 16, 4, 4),
		new BinOp("minus", 3, 4, -1),
		new BinOp("multiplication", 3, 4, 12),
		new BinOp("plus", 3, 4, 7),
	]
	for (let op of binOperations) {
		it(op.name, async () => {
			await program.methods
				.executeBinOp(
					{[op.name]: {}},
					new anchor.BN(op.lhs),
					new anchor.BN(op.rhs),
				)
				.accounts({
					calculator: calculatorPair.publicKey,
				})
				.rpc()
			const account = await program.account.calculator.fetch(calculatorPair.publicKey)
			expect(account.result).eq(new anchor.BN(op.res))
		})
	}
})
