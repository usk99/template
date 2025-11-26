import type { FC } from "react";
import { useState } from "react";
import type { VisualizerSettingInfo } from "../../types";

type FileUploaderProps = {
    setVisualizerSettingInfo: React.Dispatch<
        React.SetStateAction<VisualizerSettingInfo>
    >;
};

type FileSet = {
    seed: number;
    seedStr: string;
    inFile?: File;
    outFile?: File;
    errFile?: File;
};

const FileUploader: FC<FileUploaderProps> = ({ setVisualizerSettingInfo }) => {
    const [selectDisabled, setSelectDisabled] = useState(true);
    const [fileSets, setFileSets] = useState<FileSet[]>([]);

    const readFileAsText = (file: File): Promise<string> => {
        return new Promise((resolve) => {
            const reader = new FileReader();
            reader.onload = () => resolve(reader.result as string);
            reader.readAsText(file);
        });
    };

    const loadFileSet = async (fileSet: FileSet) => {
        const [inputText, outputText, errText] = await Promise.all([
            fileSet.inFile
                ? readFileAsText(fileSet.inFile)
                : Promise.resolve("init_input"),
            fileSet.outFile
                ? readFileAsText(fileSet.outFile)
                : Promise.resolve("init_output"),
            fileSet.errFile
                ? readFileAsText(fileSet.errFile)
                : Promise.resolve("init_err"),
        ]);

        setVisualizerSettingInfo((prev) => ({
            ...prev,
            seed: fileSet.seed,
            input: inputText,
            output: outputText,
            err: errText,
        }));
    };

    const onFolderUpload = (e: React.ChangeEvent<HTMLInputElement>) => {
        if (e.target.files === null) return;
        const uploadedFiles = Array.from(e.target.files);

        // ファイルをin/out/errフォルダごとに分類
        const inFiles = new Map<string, File>();
        const outFiles = new Map<string, File>();
        const errFiles = new Map<string, File>();

        for (const file of uploadedFiles) {
            const path = file.webkitRelativePath;
            const fileName = file.name;
            // ファイル名から数字部分（seed）を抽出
            const match = fileName.match(/^(\d+)/);
            if (!match) continue;
            const seedStr = match[1];

            if (path.includes("/in/")) {
                inFiles.set(seedStr, file);
            } else if (path.includes("/out/")) {
                outFiles.set(seedStr, file);
            } else if (path.includes("/err/")) {
                errFiles.set(seedStr, file);
            }
        }

        // inFilesのみを対象にする
        const newFileSets: FileSet[] = Array.from(inFiles.keys())
            .sort()
            .map((seedStr) => ({
                seed: parseInt(seedStr, 10),
                seedStr,
                inFile: inFiles.get(seedStr),
                outFile: outFiles.get(seedStr),
                errFile: errFiles.get(seedStr),
            }));

        if (newFileSets.length > 0) {
            setSelectDisabled(false);
            setFileSets(newFileSets);
            loadFileSet(newFileSets[0]);
        }
    };

    const onSelectFile = (e: React.ChangeEvent<HTMLSelectElement>) => {
        const fileSet = fileSets.find((fs) => fs.seedStr === e.target.value);
        if (fileSet !== undefined) {
            loadFileSet(fileSet);
        }
    };

    const otherAtt = {
        directory: "",
        webkitdirectory: "",
    };

    return (
        <>
            <p>
                <label>
                    File:
                    <select disabled={selectDisabled} onChange={onSelectFile}>
                        {fileSets.map((fs, index) => (
                            <option key={`option-${index}`} value={fs.seedStr}>
                                {fs.seedStr}
                            </option>
                        ))}
                    </select>
                </label>
                <input type="file" onChange={onFolderUpload} {...otherAtt} />
                <span
                    style={{
                        marginLeft: "10px",
                        fontSize: "12px",
                        color: "#666",
                    }}
                >
                    ※ in/, out/, err/ フォルダを含む親フォルダを選択
                    <br />[{fileSets.length}ファイル]
                </span>
            </p>
        </>
    );
};

export default FileUploader;
